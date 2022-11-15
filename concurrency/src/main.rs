mod channels;
mod index;
mod shared_state;

/// There are a lot of idioms for Concurrent programming:
/// * A background thread that has a single job and periodically wakes up to do it.
/// * General-purpose worker pools that communicate with clients via task queues.
/// * Pipelines where data flows from one thread to the next, with each thread doing a little of the work.
/// * Data parallelism, where it is assumed (rightly or wrongly) that the whole computer will mainly
/// just be doing one large computation, which is therefore split into n pieces and run on n threads
/// in the hopes of putting all n of the machine's cores to work at once.
/// * A sea of synchronized objects, where multiple threads have access to the same data, and races
/// are avoided using ad hoc locking schemes based on low-level primitives like mutexes.
/// * Atomic integer operations allow multiple cores to communicate by passing information through
/// fields the size of one machine word. (This is even harder to get right than all the others, unless
/// the data being exchanged is literally just integer values. In practice, it's usually pointers.)
fn main() {
    println!("Hello, world!");
}


