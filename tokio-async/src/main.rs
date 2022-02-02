use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {

            let (reader, mut write) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                write.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
