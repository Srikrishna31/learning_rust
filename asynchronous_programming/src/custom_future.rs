use std::sync::{Arc, Mutex};
use std::task::{Waker, Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub fn spawn_blocking<T, F>(closure: F) -> SpawnBlocking<T>
where F: FnOnce() -> T,
      F: Send + 'static,
      T: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    std::thread::spawn({
        let inner = inner.clone();
        move || {
            let value = closure();
            let maybe_waker = {
                let mut guard = inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };

            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });

    SpawnBlocking(inner)
}


/// `SpawnBlocking<T>` is a future of the closure's return value.
pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

/// The `Shared` struct must serve as a rendezvous between the future and the thread running the closure,
/// so it is owned by an `Arc` and protected with a `Mutex`. Polling the future checks whether `value`
/// is present and saves the waker in `waker` if not. The thread that runs the closure saves its
/// return value in `value` and then invokes `waker` if present.
struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}


impl<T: Send> Future for SpawnBlocking<T> {
    type Output =T;

    /// Once a `Future` has returned `Poll::ready`, you're not supposed to poll it again. The usual
    /// ways of consuming futures, like `await` and `block_on`, all respect this rule. If a
    /// `SpawnBlocking` future is overpolled, nothing especially terrible happens, but it doesn't go
    /// to any effort to handle that case, either. This is typical for handwritten futures.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            return Poll::Ready(value);
        }

        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

use waker_fn::waker_fn;
use futures_lite::pin;
use crossbeam::sync::Parker;
/// # Implementing block_on
/// The `crossbeam` crate's `Parker` type is a simple blocking primitive: calling `parker.park()`
/// blocks the thread until someone else calls `.unpark()` on the corresponding `Unparker`, which you
/// obtain beforehand by calling `parker.unparker()`. If you `unpark` a thread that isn't parked yet,
/// its next call to `park` returns immediately, without blocking. Our `block_on` will use the `Parker`
/// to wait whenever the future isn't ready, and the waker we pass to futures will unpark it.
///
/// The `waker_fn` function, from the crate of the same name, creates a `Waker` from a given closure.
///
///     pin!(future)
/// Given a variable holding a future of type F, the pin! macro takes ownership of the future and
/// declares a new variable of the same name whose type is `Pin<&mut F>` and that borrows the future.
/// This gives us the `Pin<&mut Self>` required by the `poll` method.
fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move ||unparker.unpark());
    let mut context = Context::from_waker(&waker);

    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

use std::{io, net};

/// # Pinning
/// Although asynchronous functions and blocks are essential for writing clear asynchronous code,
/// handling their futures requires a bit of care. The `Pin` type helps Rust ensure they're used safely.
///
/// In a synchronous function, all local variables live on the stack, but in an asynchronous function,
/// local variables that are alive across an `await` must be located in the future, so they'll be
/// available when it is polled again. Borrowing a reference to a local variable borrows a part of
/// the future.
///
/// Every future has two life stages:
/// * The first stage begins when the future is created. Because the function's body hasn't begun
/// execution, no part of it could possibly be borrowed yet. At this point, it's as safe to move as
/// any other Rust value.
/// * The second stage begins the first time the future is polled. Once the function's body has begun
/// execution, it could borrow references to variables stored in the future and then await, leaving
/// that part of the future borrowed. Starting after its first poll, we must assume the future may
/// not be safe to move.
///
/// To enter its second life stage, the future must be polled. The `poll` method requires the future
/// to be passed as a `Pin<&mut Self>` value. `Pin` is a wrapper for pointer types that restricts how
/// the pointers can be used, ensuring that their referents (like Self) cannot every be moved again.
/// So you must produce a `Pin`-wrapped pointer to the future before you can poll it.
///
/// This is Rust's strategy for keeping futures safe: a future can't become dangerous to move until
/// it's polled; you can't poll a future until you've constructued a `Pin`-wrapped pointer to it; and
/// once you've done that, the future can't be moved.
async fn fetch_string(address: &str) -> io::Result<String> {
    // 1
    let mut socket = net::TcpStream::connect(address).await/*2*/?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await/*3*/?;
    Ok(buf)
}

/// # Pinned Pointers
/// The `Pin` type is a wrapper for pointers to futures that restricts how the pointers may be used
/// to make sure that futures can't be moved once they've been polled.
/// By *pointer*, we mean any type that implements `Deref`, and possibly `DerefMut`. A `Pin` wrapped
/// around a pointer is called a *pinned pointer*. `Pin<&mut T>` and `Pin<Box<T>>` are typical. The
/// definition of `Pin` in the standard library is simple:
///
///     pub struct Pin<P> {
///         pointer: P,
///     }
/// Note that the pointer field is *not* `pub`. This means that the only way to construct or use a
/// `Pin` is through the carefully chosen methods the type provides.
///
/// Given a future of an asynchronous function or block, there are only a few ways to get a pinned
/// pointer to it:
/// * The `pin!` macro, from the futures-lite crate, shadows a variable of type T with a new one of
/// type `Pin<&mut T>`. The new variable points to the original's value, which has been moved to an
/// anonymous temporary location on the stack. When the variable goes out of scope, the value is
/// dropped.
/// * The standard library's `Box::pin` constructor takes ownership of a value of any type T, moves
/// it into the heap, and returns a Pin<Box<T>>.
/// * `Pin<Box<T>>` implements `From<Box<T>>`, so `Pin::from(boxed)` takes ownership of `boxed` and
/// gives you back a pinned box pointing at the same T on the heap.
///
/// Every way to obtain a pinned pointer to these futures entails giving up ownership of the future,
/// and there is no way to get it back out. The pinned pointer itself can be moved in any way you
/// please, but moving a pointer doesn't move its referent. So possession of a pinned pointer to a
/// future serves as proof that you have permanently given up the ability to move that future. This
/// is all we need to know that it can be polled safely.
///
/// # The Unpin Trait
/// It is a marker trait, for which `Pin` imposes no restrictions whatsoever. You can make a pinned
/// pointer from an ordinary pointer with `Pin::new` and get the pointer back out with `Pin::into_inner`.
/// The `Pin` itself passes along the pointer's own `Deref` and `DerefMut` implementations.
fn unpin_example() {
    let mut string = "Pinned?".to_string();
    let mut pinned: Pin<&mut String> = Pin::new(&mut string);

    pinned.push_str(" Not");
    Pin::into_inner(pinned).push_str(" so much.");

    let new_home = string;
    assert_eq!(new_home, "Pinned? Not so much.");
}
