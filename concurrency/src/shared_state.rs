type PlayerId = u32;
const GAME_SIZE: usize=8;

type WaitingList = Vec<PlayerId>;

use std::sync::Mutex;

struct FernEmpireApp {
    id: PlayerId,
    /// Unlike C++, in Rust the protected data is stored inside the Mutex.
    /// The only way to get at the data is to call the .lock() method. The return value ven lets us
    /// borrow direct references to the underlying data. Rust's lifetime system ensures those references
    /// can't outlive the guard itself. There is no way to access the data in a Mutex without holding
    /// the lock.
    /// When a guard is dropped, the lock is released. Ordinarily that happens at the end of the block
    /// but you can also drop it manually.
    /// Rust's mutex provides exclusive access (mut) to the data inside, even though many threads
    /// may have shared (non-mut) access to the Mutex itself.
    /// Mutex and RefCell are both flavors of interior mutability.
    waiting_list: Mutex<WaitingList>,
}

impl FernEmpireApp {
    /// Add a player to the waiting list for the next game. Start a new game immediately if enough
    /// players are waiting.
    fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();

        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            drop(guard);
            self.start_game(players);
        }
    }

    fn start_game(&self, players: Vec<PlayerId>) {

    }
}

use std::sync::{Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
/// Safe Rust code cannot trigger a data race, a specific kind of bug where multiple thread read and
/// write the same memory concurrently, producing meaningless results.
/// However, threads that use mutexes are subject to some other problems that Rust doesn't fix for you:
/// * Valid Rust programs can't have data races, but they can still have other race conditions - situations
/// where a program's behavior depends on timing among threads and may therefore vary from run to run.
/// Some race conditions are benign. Some manifest as general flakiness and incredibly hard-to-fix bugs.
/// Using mutexes in an unstructured way invites race conditions. It's upto you to make sure they're
/// benign.
/// * Shared mutable state also affects program design. Mutexes encourage a "just-add-a-method" way of
/// working that can lead to a monolithic blob of interrelated code.
/// * Mutexes are just not as simple as they seem.
///
/// A thread can deadlock itself by trying to acquire a lock that it's already holding. To put it another
/// way, the lock in a Mutex is not a recursive lock. There are other ways to get deadlock, too, involving
/// multiple threads that each acquire multiple mutexes at once. Rust's borrow system can't protect
/// you from deadlock. The best protection is to keep critical sections small: get in, do your work and
/// get out.
/// It's also possible to get a deadlock with channels. For example, two threads might block each one
/// waiting to receive a message from the other. However, again, good program design can give you high
/// confidence that this won't happen in practice.
///
/// Poisoned Mutexes
/// If a thread panics while holding a Mutex, Rust marks the Mutex as poisoned. Any subsequent attempt
/// to lock the poisoned Mutex will get an error result.
/// The reason mutexes are poisoned on panic is not for fear of undefined behavior. Rather, the concern
/// is that you've probably been programming with invariants. Since your program panicked and bailed
/// out of a critical section without finishing what it was doing, perhaps having updated some fields
/// of the protected data but not others, it's possible that the invariants are now broken. Rust poisons
/// the mutex to prevent other threads from blundering unwittingly into this broken situation and
/// making it worse. You can still lock a poisoned mutex and access the data inside, with mutual exclusion
/// fully enforced. But you won't do it by accident.
///
/// Multiconsumer Channels using Mutexes
/// We can add a Mutex around a Receiver and share it to make it multiconsumer channel
#[derive(Clone)]
pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

impl<T> Iterator for SharedReceiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = self.0.lock().unwrap();
        guard.recv().ok()
    }

}

/// Create a new channel whose receiver can be shared across threads. This returns a sender and α
/// receiver, just like the stdlib's `channel()`, and sometimes works as a drop-in replacement.
pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
    let (sender, receiver) = channel();
    (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
}
