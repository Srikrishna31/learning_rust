use async_std::prelude::*;
use async_chat::utils::{self, ChatResult};
use async_chat::{FromClient, FromServer};
use async_std::{io, net, task};
use std::sync::Arc;

/// # Asynchronous Streams
/// A *stream* is the asynchronous analogue of an iterator: it produces a sequence of values on demand,
/// in an async-friendly fashion. Here's the definition of the `Stream` trait, from the async-std::stream
/// module:
///
///     trait Stream {
///         type Item;
///
///         fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
///     }
///
/// You can look at this as a hybrid of the Iterator and Future traits. Like an iterator, a `Stream`
/// has an associated `Item` type an uses `Option` to indicate when the sequence has ended. But like
/// a future, a stream must be polled: to get the next item (or learn that the stream has ended), you
/// must call poll_next until it returns `Poll::Ready`. A stream's `poll_next` implementation should
/// always return quickly, without blocking. And if a stream returns `Poll::Pending`, it must notify
/// the caller when it's worth polling again via the `Context`.
///
/// The `poll_next` method is awkward to use directly, but you won't generally need to do that. Like
/// iterators, streams have a broad collection of utility methods like filter and map. Among these is
/// a `next` method, which returns a future of the stream's next `Option<Self::Item>`. Rather than
/// polling the stream explicitly, you can call `next` and await the future it returns instead.
///
/// When working with streams, it's important to remember to use the async_std prelude. This is because
/// the utility methods for the `Stream` trait, like `next`, `map`, `filter`, and so on, are actually
/// not defined on `Stream` itself. Instead, they are default methods of a separate trait, `StreamExt`,
/// which is automatically implemented for all `Streams`:
///
///     pub trait StreamExt: Stream {
///         ... define utility methods as default methods ...
///     }
///
///    impl<T: Stream> StreamExt for T {}
async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!("Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows)\
              to close the connection.");

    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;

        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;
    if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        return Some(FromClient::Post{
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        });
    } else if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        return Some(FromClient::Join {
            group_name: Arc::new(group.to_string()),
        });
    } else {
        eprintln!("Unrecognized command: {:?}", line);
        return None;
    }
}

/// Given a string `input`, return `Some((token, rest))`, where `token` is the first run of non-whitespace
/// characters in `input`, and `rest` is the rest of the string. If the string contains no non-whitespace
/// characters, reutn `None`.
fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}

/// This function takes a socket receiving data from the server, wraps a `BufReader` around it, and
/// then passes that to `receive_as_json` to obtain a stream of incoming `FromServer` values. Then it
/// uses a `while let` loop to handle incoming replies, checking for error results and printing each
/// server reply for the user to see.
async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);

    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message { group_name, message} => {
                println!("message posted to {group_name}: {message}");
            }
            FromServer::Error(message) => {
                println!("error from server: {message}");
            }
        }
    }

    Ok(())
}

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodely(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}
