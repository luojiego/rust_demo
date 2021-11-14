use anyhow::Result;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("localhost:9710").await?;
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    stream.send(Bytes::from("hello world")).await?;

    if let Some(Ok(data)) = stream.next().await {
        println!("Got: {:?}", String::from_utf8_lossy(&data));
    }

    Ok(())
}
