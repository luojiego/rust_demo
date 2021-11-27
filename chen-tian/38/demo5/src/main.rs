use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());
            let (mut w, mut r) = framed.split();
            for line in r.next().await {
                w.send(format!("I got: {}", line?)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
