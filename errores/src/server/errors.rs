use std::io::Error;

#[derive(Debug)]
pub enum ServerError {
    InitError,
    ConnectionError(Error),
    WriteError(Error),
}
