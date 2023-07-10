use rendevouz::run;
use std::net::{TcpListener};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.1:8000")?;
    run(address)?.await
}
