use std::process::exit;

use errores::server::{errors::ServerError, Server};

fn run() -> Result<(), ServerError> {
    let server = Server::new()?;
    server.run()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{:?}", err);
        exit(1);
    }
}
