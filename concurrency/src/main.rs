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
