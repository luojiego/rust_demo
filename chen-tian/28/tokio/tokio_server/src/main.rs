use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:9710").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted: {:?}", addr);

        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            while let Some(Ok(data)) = stream.next().await {
                println!("Got: {:?}", String::from_utf8_lossy(&data));

                stream.send(Bytes::from("goodbye world!")).await.unwrap();
            }
        });
    }
}
