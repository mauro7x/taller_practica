mod config;
mod constants;
pub mod errors;

use self::{config::Config, constants::*, errors::ServerError};
use std::{io::Write, net::TcpListener};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> Result<Server, ServerError> {
        let config = Config::new().map_err(|_e| ServerError::InitError)?;
        let addr = format!("0.0.0.0:{}", config.port);
        let listener = TcpListener::bind(addr).map_err(|_e| ServerError::InitError)?;
        let server = Server { listener };

        Ok(server)
    }

    pub fn run(&self) -> Result<(), ServerError> {
        println!("Accepting connections...");
        for connection in self.listener.incoming() {
            let mut stream = connection.map_err(|e| ServerError::ConnectionError(e))?;

            if let Err(err) = stream.write_all(WELCOME_MSG.as_bytes()) {
                println!("Falle escribiendo en un socket. Error: {}", err);
            }
        }

        Ok(())
    }
}
