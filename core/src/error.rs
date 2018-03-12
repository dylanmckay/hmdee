/// A headset error.
#[derive(Debug, Fail)]
pub enum Error {
    /// There was an error whilst communicating with the headset.
    #[fail(display = "communication error: {}", message)]
    CommunicationError {
        message: String,
    }
}

