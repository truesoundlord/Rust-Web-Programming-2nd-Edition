use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    println!("stream starting");
    stream.write_all(b"8.0;BYND;\n9.0;PLTR").await?;
    println!("stream finished");
    Ok(())
}