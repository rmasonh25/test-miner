use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3333";

    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            println!("[Miner] Connected to {}", addr);

            let msg = r#"{"id": 1, "method": "mining.subscribe", "params": []}"#;
            stream.write_all(format!("{}\n", msg).as_bytes()).await.unwrap();

            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 { break; }
                println!("[Miner] Got: {}", line.trim());
                line.clear();
            }
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
        }
    }
}
