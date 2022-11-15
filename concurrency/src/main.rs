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


use std::{fs, io, thread};
use std::path::PathBuf;
use std::sync::mpsc;

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
