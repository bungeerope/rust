use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:12345";
    let listener = TcpListener::bind(&address).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buff = [0; 1024];
            loop {
                let n = match socket.read(&mut buff).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket, err is {:?}", e);
                        return;
                    }
                };
                print!("receive msg: {}", String::from_utf8_lossy(&buff[0..n]));
                if let Err(e) = socket.write_all(&buff[0..n]).await {
                    eprintln!("failed to write to socket, err is {:?}", e);
                    return;
                };
            }
        });
    }
}