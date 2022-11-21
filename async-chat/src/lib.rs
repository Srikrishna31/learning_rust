use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

/// The `FromClient` enum represents the packets a client can send to the server: it can ask to join
/// a group and post messages to any group it has joined.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {group_name: Arc<String>},
    Post {
        group_name: Arc<String>,
        message: Arc<String>
    }
}

/// `FromServer` represents what the server can send back: messages posted to some group, and error
/// messages. Using a reference-counted `Arc<String>` instead of a plain `String` helps the server
/// avoid making copies of strings as it manages groups and distributes messages.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>
    },
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromclient_json() {
        let from_client = FromClient::Post {
            group_name: Arc::new("Dogs".to_string()),
            message: Arc::new("Samoyeds rock!".to_string()),
        };

        let json = serde_json::to_string(&from_client).unwrap();

        assert_eq!(json,
                    r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#);

        assert_eq!(serde_json::from_str::<FromClient>(&json).unwrap(), from_client);
    }
}
