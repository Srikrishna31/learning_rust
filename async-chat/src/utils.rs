use std::error::Error;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;
use serde::de::DeserializeOwned;

/// This function builds the JSON representation of packet as a `String`, adds a newline to the end,
/// and then writes it all to outbound.
///
/// From its `where` clause, you can see that the `send_as_json` is quite flexible. The type of packet
/// to be sent, P, can be anything that implements `serde::Serialize`. The output stream S can be
/// anything that implements `async_std::io::Write`, the asynchronous version of the `std::io::Write`
/// trait for output streams. This is sufficient for us to send `FromClient` and `FromServer` values
/// on an asynchronous `TcpStream`. Keeping the defintion of `send_as_json` generic ensures that it
/// doesn't depend on the details of the stream or packet types in surprising ways: `send_as_json`
/// can only use methods from those traits.
pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

/// Like `send_as_json`, this function is generic in the input stream and packet types:
/// * The stream type S must implement `async_std::io::BufRead`, the asynchronous analogue of
/// `std::io::BufRead`, representing a buffered input byte stream.
/// * The packet type P must implement `DeserializeOwned`, a stricter variant of serde's `Deserialize`
/// trait. For efficiency, `Deserialize` can produce &str and &[u8] values that borrow their contents
/// directly from the buffer they were deserialized from, to avoid copying data. In our case, however,
/// we need to return the deserialized values to our caller, so they must be able to outlive the
/// buffers we parsed from them. A type that implements `DeserializeOwned` is always independent of
/// the buffer it was deserialized from.
pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
    where S: async_std::io::BufRead + Unpin,
          P: DeserializeOwned,
{
    inbound.lines()
        .map(|line_result| -> ChatResult<P> {
            let line = line_result?;
            let parsed = serde_json::from_str::<P>(&line)?;
            Ok(parsed)
        })
}
