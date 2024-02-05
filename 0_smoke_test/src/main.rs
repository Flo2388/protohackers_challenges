use std::io;
use tokio::net::TcpStream;

use tokio::net::TcpSocket;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

async fn process_stream(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer: Vec<u8>= Vec::new();

    let _read_bytes: usize = stream.read_to_end(&mut buffer).await?;
    println!("{:?}", buffer);

    stream.write_all(&buffer).await?;
    stream.flush().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    socket.bind(addr)?;

    let listener = socket.listen(1024)?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        process_stream(&mut stream).await?;
    }

//    Ok(())
}
