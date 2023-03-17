use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if let Ok(num_bytes) = io::copy(&mut rd, &mut wr).await {
                println!("Copied {num_bytes} bytes");
            } else {
                eprintln!("failed to copy");
            }
        });
    }
}
