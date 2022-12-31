use std::fmt::format;
use zero2prod::run;
use zero2prod::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener =
        std::net::TcpListener::bind(address)?;
    run(listener)?.await
}
