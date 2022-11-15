use std::{fs, io, thread};
use std::path::{PathBuf, Path};
use std::sync::mpsc;

use crate::index::InMemoryIndex;

/// Fork-Join Parallelism
/// The simplest use cases for threads arise when we have several completely independent tasks that
/// we'd like to do at once. This pattern is called fork-join parallelism. To fork is to start a new
/// thread, and to join a thread is to wait for it to finish.
/// Fork-join parallelism is attractive for a few reasons:
/// * It's dead simple. Fork-join is easy to implement, and Rust makes it easy to get right.
/// * It avoids bottlenecks. There's no locking of shared resources in fork-join. The only time any
/// thread has to wait for another is at the end. In the meantime, each thread can run freely. This
/// helps keep task-switching overhead low.
/// * The performance math is straightforward. In the best case, by starting four threads, we can
/// finish our work in a quarter of the time. There are a couple reasons for not the ideal speedup:
/// we might not be able to distribute the work evenly across all threads. Another reason for caution
/// is that sometimes fork-join programs must spend time after the threads join combining the reuslts
/// computed by the threads. That is, isolating the tasks completely may make the some extra work.
/// Still, apart from these two things, any CPU-bound program with isolated units of work can expect
/// a significant boost.
/// * It's easy to reason about program correctness. A fork-join program is deterministic as long as
/// the threads are really isolated. The program always produces the same result, regardless of
/// variations in thread speed. It's a concurrency model without race conditions.
///
/// The main disadvantage of fork-join is that it requires isolated units of work.
///
/// Channels
/// A channel is a one-way conduit for sending values from one thread to another. In other words, it's
/// a thread safe queue. They're something like Unix pipes: one end is for sending data, and the other
/// is for receiving. The two ends are typically owned by two different threads. But whereas Unix pipes
/// are for sending bytes, channels are for sending Rust values. sender.send(item) puts a single value
/// into the channel; receiver.recv() removes one. Ownership is transferred from the sending thread to
/// the receiving thread. If the channel is empty, receiver.recv() blocks until a value is sent.
/// With channels, threads can communicate by passing values to one another. It's a very simple way for
/// threads to work together without using locking or shared memory.
/// The send and recv methods both return Results, but these methods fail only if the other end of the
/// channel has been dropped. A send call fails if the Receiver has been dropped, because otherwise
/// the value would sit in the channel forever: without a Receiver, there's no way for any thread to
/// receive it. Likewise, a recv call fails if there are no values waiting in the channel and the sender
/// has been dropped, because otherwise recv would wait forever: without a Sender, there's no way for
/// any thread to send the next value. Dropping your end of a channel is the normal way of "hanging up",
/// closing the connection when you're done with it.
fn start_file_reader_thread(documents: Vec<PathBuf>)
                            -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>)
{
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;

            if sender.send(text).is_err() {
                break;
            }
        }

        Ok(())
    });

    (receiver, handle)
}


fn start_file_indexing_thread(texts: mpsc::Receiver<String>)
    -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>)
{
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);

            if sender.send(index).is_err() {
                break;
            }
        }
    });

    (receiver, handle)
}


fn start_in_memory_merge_thread(file_indexes: mpsc::Receiver<InMemoryIndex>)
    -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        //dummy implementation for merge now.
        let new_index = InMemoryIndex::new();
        if sender.send(new_index).is_err() {

        }
    });
    (receiver, handle)
}

fn start_index_writer_thread(big_indexes: mpsc::Receiver<InMemoryIndex>, output_dir: &Path)
    -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        //dummy implementation for write now.
        let path = PathBuf::new();
        if sender.send(path).is_err() {
            //return Err("Error writing index");
        }
        Ok(())
    });
    (receiver, handle)
}

/// This last stage doesnot return a Receiver, because it's the end of the line. It produces a single
/// output file on disk. It doesn't return a JoinHandle, because we don't bother spawning a thread
/// for this stage. The work is done on the caller's thread.
fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path) -> io::Result<()>
{
    Ok(())
}


fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()>
{
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_files(files, &output_dir);

    // Wait for threads to finish, holding on to any errors that they encounter.
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();

    // Return the first error encountered if any. (As it happens, h2 and h3 can't fail: those threads
    // are pure in-memory data processing.)
    r1?;
    r4?;
    result
}


/// Channel Features and Performance
/// The mpsc part of std::sync::mpsc stands for multiproducer, single-consumer, a terse description
/// of the kind of communication Rust's channels provide.
/// Sender<T> implements the Clone trait. To get a channel with multiple senders, simply create a
/// regular channel and clone the sender as many times as you like. You can move each Sender value
/// to a different thread.
/// A Receiver<T> can't be cloned, so if you need to have multiple threads, receiving values from the
/// same channel, you need a Mutex.
/// Rust channels are carefully optimized. When a channel is first created, Rust uses a special "one-
/// shot" queue implementation. If you only ever send one object through the channel, the overhead
/// is minimal. If you send a second value, Rust switches to a difference queue implementation. It's
/// settling in for the long haul, really, preparing the channel to transfer many values while
/// minimizing allocation overhead. And if you clone the Sender, Rust must fall back on yet another
/// implementation, one that is safe when multiple threads are trying to send values at once. But even
/// the slowest of these three implementations is lock-free queue, so sending or receiving a value is
/// at most a few atomic operations and a heap allocation, plus the move itself. System calls are needes
/// only when the queue is empty and the receiving thread therefore needs to put itself to sleep. In
/// this case, of course, traffic through your channel is not maxed out anyway.
///
/// Despite all that optimization work, there is one mistake that's very easy for applications to make
/// around channel performance: sending values faster than they can be received and processed. This
/// causes an ever-growing backlog of values to accumulate in the channel. This kind of misbehavior
/// costs memory and hurts locality. Even worse, the sending thread keeps running, using up CPU and
/// other system resources to send ever more values, just when those resources are most needed on the
/// receiving end.
///
/// Here Rust again takes a page from Unix pipes. Unix uses an elegant trick to provide some
/// backpressure so that fast senders are forced to slow down: each pipe on a Unix system has a fixed
/// size, and if a process tries to write to a pipe that's momentarily full, the system simply blocks
/// that process until there's room in the pipe. The Rust equivalent is called a synchronous channel:
///
///     use std::sync::mpsc;
///     let (sender, receiver) = mpsc::sync_channel(1000);
///
/// A synchronous channel is exactly like a regular channel except that when you create it you specify
/// how many values it can hold. For a synchronous channel, sender.send(value) is potentially a
/// blocking operation.
///
/// Thread Safety: Send and Sync
/// Rust's full thread safety story hinges on two built-in traits, std::marker::Send and
/// std::marker::Sync.
/// * Types that implement Send are safe to pass by value to another thread. They can be moved across
/// threads.
/// * Types that implement Sync are safe to pass by non-mut reference to another thread. They can be
/// shared across threads.
///
/// Safe here means: free from data races and other undefined behavior.
/// The fact that Vec<String> implements Send is an API promise that, the allocator used internally
/// by Vec and String is thread-safe, which means allocation can happen on one thread and free can
/// happen on another thread.
/// Most types are both Send and Sync. You don't even have to use #[derive] to get these traits on
/// structs and enums in your program. Rust does it for you. A struct or enum is Send if its fields
/// are Send, and Sync if its fields are Sync.
/// Some types are Send, but not Sync. This is generally on purpose, as in the case of mpsc::Receiver,
/// where it guarantees that the receiving end of an mpsc channel is used by only one thread at a time.
/// The few types that are neither Send nor Sync are mostly those that use mutability in a way that
/// isn't thread-safe. Eg. std::rc::Rc.
///
/// Send and Sync appear as bounds in the type signature of functions that transfer data across
/// thread boundaries. When you spawn a thread, the closure you pass must be Send, which means all
/// the values it contains must be Send. Similarly, if you want to send values through a channel to
/// another thread, the values must be Send.
pub trait OffThreadExt: Iterator {
    /// Transform this iterator into an off-thread iterator: the `next()` calls happen on a separate
    /// worker thread, so the iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
    where T: Iterator + Send + 'static,
          T::Item: Send + 'static
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        // Create a channel to transfer items from the worker thread.
        let (sender, receiver) = mpsc::sync_channel(1024);

        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        //Return an iterator that pulls values from the channel.
        receiver.into_iter()
    }
}

/// Channels can also be used for cases where one thread sends a request to another thread and needs
/// to get some sort of response back. The first thread's request can be a struct or tuple that includes
/// a Sender, a sort of self-addressed envelope that the second thread uses to send its reply. The
/// first thread gets to decide whether to block and wait for the response or use the .try_recv()
/// method to poll for it.
struct Dummy;
