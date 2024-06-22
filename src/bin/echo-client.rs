

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    let mut buf = vec![0; 1024];
    stream.write_all(b"hello world!").await?;
    let n = stream.read(&mut buf).await?;
    let recv_msg:String = String::from_utf8(buf[0..n].to_vec()).unwrap();
    println!("Received: {}", recv_msg);
    Ok(())
}
