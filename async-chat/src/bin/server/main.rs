use async_std::prelude::*;
use async_chat::utils::ChatResult;
use std::sync::Arc;
use tokio::net;

mod group_table;
mod connection;
mod group;

use connection::serve;

/// The server's `main` function resembles the client's: it does a little bit of setup and then calls
/// `block_on` to run an async block that does the real work. To handle incoming connections from
/// clients, it creates a `TcpListener` socket, whose `incoming` method returns a stream of
/// `std::io::Result<TcpStream>` values.
///
/// For each incoming connection, we spawn an asynchronous task running the `connection::serve`
/// function. Each task also receives a reference to a `GroupTable` value representing our server's
/// current list of chat groups, shared by all the connections via an Arc reference-counted pointer.
///
/// If `connection::serve` returns an error, we log a message to the standard error output and let
/// the task exit. Other connections continue to run as usual.
fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");

    let chat_group_table = Arc::new(group_table::GroupTable::new());

    async_std::task::block_on(async {
        use async_std::{net, task};

        let listener = net::TcpListener::bind(address).await?;

        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {error}");
    }
}
