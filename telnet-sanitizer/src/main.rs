use safe_telnet_parser::TelnetEvent;
use safe_telnet_parser::TelnetParser;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2323").await?;
    let mut parser = TelnetParser::new(); // must be mutable now

    loop {
        let (socket, _) = listener.accept().await?;
        let mut buf = [0u8; 1024];

        socket.readable().await?;
        let n = socket.try_read(&mut buf)?;
        let events = parser.parse(&buf[..n]);

        for e in events {
            if let TelnetEvent::Data(b) = e {
                println!("Safe byte: {}", b);
            }
        }
    }
}
