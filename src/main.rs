use std::net::TcpListener;

use rendevouz::startup::run;
use rendevouz::configuration::get_configuration;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("{:?}", listener.local_addr());
    run(listener)?.await
}
