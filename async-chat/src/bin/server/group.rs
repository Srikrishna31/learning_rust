use async_std::task;
use crate::connection::Outbound;
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::error::RecvError};
use async_chat::FromServer;

/// In our server, the group::Group type represents a chat group. This type only needs to support the
/// two methods that connection::serve calls: `join` to add a new member, and `post` to post a new
/// message. Each message posted needs to be distributed to all the members. The challenge of
/// *backpressure* is handled as below:
/// * If one member can't keep up with the messages being posted to the group-if they have a slow
/// network connection, for example - other members in the group should not be affected.
/// * Even if a member falls behind, there should be means for them to rejoin the conversation and
/// continue to participate somehow.
/// * Memory spent buffering messages should not grow without bound.
///
/// Because these challenges are common when implementing many-to-many communication patterns, the
/// `tokio` crate provides a *broadcast channel* type that implements one reasonable set of tradeoffs.
/// A `tokio` broadcast channel is a queue of values that allows any number of different threads or
/// tasks to send and receive values. It's called a "broadcast" channel because every consumer gets
/// its own copy of each value sent. (The value type must implement `Clone`.)
///
/// Normally, a broadcast channel retains a message in the queue until every consumer has gotten their
/// copy. But if the length of the queue would exceed the channel's maximum capacity, specified when
/// it is created, the oldest messages get dropped. Any consumers who couldn't keep up get an error
/// the next time they try to get their next message, and the channel catches them up to the oldest
/// message still available.
pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>
}

impl Group {
    pub fn new(name: Arc<String>) -> Group {
        let (sender, _receiver) = broadcast::channel(1000);
        Group{name, sender}
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();

        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        // This only returns an error when there are no subscribers. A connection's outgoing side can
        // exit, dropping its subscription, slightly before its incoming side, which may end up trying
        // to send a message to an empty group.
        let _ignored = self.sender.send(message);
    }
}


async fn handle_subscriber(group_name: Arc<String>, mut receiver: broadcast::Receiver<Arc<String>>,
                            outbound: Arc<Outbound>)
{
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message:message.clone(),
            },
            Err(RecvError::Lagged(n)) => FromServer::Error(
                format!("Dropped {n} messages from {group_name}.")
            ),
            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
