use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use telnet_sanitizer::TelnetSanitizer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2323").await?;
    println!("🚨 Telnet sanitizer server listening on 127.0.0.1:2323");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("📡 Connection from {}", addr);

        tokio::spawn(async move {
            let mut sanitizer = TelnetSanitizer::new();
            let mut buf = [0u8; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("🔌 Client disconnected");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("❌ Read error: {}", e);
                        return;
                    }
                };

                let input = &buf[..n];
                let sanitized = sanitizer.sanitize(input);

                println!("--------------------------------");
                println!("RAW BYTES      : {:?}", input);
                println!("SANITIZED BYTES: {:?}", sanitized);
                println!("SANITIZED TEXT : {}", String::from_utf8_lossy(&sanitized));
                println!("--------------------------------");

                // Echo sanitized output back to client
                if let Err(e) = socket.write_all(&sanitized).await {
                    eprintln!("❌ Write error: {}", e);
                    return;
                }
            }
        });
    }
}
