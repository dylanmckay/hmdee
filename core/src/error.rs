use std;

/// A headset error.
#[derive(Debug, Fail)]
pub enum Error {
    /// There was an error whilst communicating with the headset.
    #[fail(display = "communication error: {}", message)]
    CommunicationError {
        message: String,
    }
}

impl Error {
    /// Creates a new communication error.
    pub fn communication_error<M>(message: M) -> Self where M: std::fmt::Display {
        Error::CommunicationError { message: message.to_string() }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::CommunicationError { message: e.to_string() }
    }
}

