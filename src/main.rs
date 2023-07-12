use rendevouz::configuration::get_configuration;
use rendevouz::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("\n Rendevouz is running at: http://{}:{}/health_check\n", configuration.address, configuration.application_port);
    run(listener, connection_pool)?.await
}
