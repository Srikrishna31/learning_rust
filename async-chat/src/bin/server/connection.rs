use async_chat::{FromClient, FromServer, utils::{self, ChatResult}};
use async_std::{prelude::*, io::BufReader, net::TcpStream, sync::{Arc, Mutex}};

use crate::group_table::GroupTable;

/// This is almost a mirror image of the client's `handle_replies` function: the bulk of the code is
/// a loop handling an incoming stream of FromClient values, built from a buffered TCP stream with
/// receive_as_json. If an error occurs, we generate a `FromServer::Error` packet to convey the bad
/// news back to the client.
pub async fn serve(socket: TcpStream, groups : Arc<GroupTable>) -> ChatResult<()>
{
    let outbound = Arc::new(Outbound::new(socket.clone()));

    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive_as_json(buffered);

    while let Some(request_result) = from_client.next().await {
        let request = request_result?;

        let result = match request {
            FromClient::Join{group_name} => {
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());
                Ok(())
            }

            FromClient::Post { group_name, message } => {
                match groups.get(&group_name) {
                    Some(group) => {
                        group.post(message);
                        Ok(())
                    }
                    None => {
                        Err(format!("Group '{group_name}' does not exist"))
                    }
                }
            }
        };

        if let Err(message) = result {
            let report = FromServer::Error(message);
            outbound.send(report).await?;
        }
    }

    Ok(())
}

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(to_client: TcpStream) -> Outbound {
        Outbound(Mutex::new(to_client))
    }

    pub async fn send(&self, packet: FromServer) -> ChatResult<()> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
