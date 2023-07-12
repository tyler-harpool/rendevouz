use rendevouz::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.1:0")?;
    println!("{:?}", address.local_addr());
    run(address)?.await
}
