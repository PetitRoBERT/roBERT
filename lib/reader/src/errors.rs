use std::io::Error;
#[derive(Debug)]
pub enum ReaderError {
    IoError(Error),
    DecodingError(String),
    NotImplemented,
}

impl From<Error> for ReaderError {
    fn from(err: Error) -> ReaderError {
        ReaderError::IoError(err)
    }
}
