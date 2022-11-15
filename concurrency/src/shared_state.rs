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

    fn start_game(players: Vec<PlayerId>) {

    }
}
