use Error;
use hidapi;

/// Provides access to system resources.
pub struct Context {
    hidapi: hidapi::HidApi,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Result<Self, Error> {
        let mut hidapi = hidapi::HidApi::new().map_err(Error::communication_error)?;
        hidapi.refresh_devices();

        Ok(Context {
            hidapi,
        })
    }

    /// Gets the HIDAPI context.
    pub(crate) fn hidapi(&self) -> &hidapi::HidApi { &self.hidapi }
}

