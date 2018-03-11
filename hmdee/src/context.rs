use hidapi;

/// Provides access to system resources.
pub struct Context {
    hidapi: hidapi::HidApi,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Context {
            hidapi: hidapi::HidApi::new().expect("failed to initialize hidapi"),
        }
    }

    /// Gets the HIDAPI context.
    pub(crate) fn hidapi(&self) -> &hidapi::HidApi { &self.hidapi }
}

