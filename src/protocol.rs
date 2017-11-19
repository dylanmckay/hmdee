
/// The size of the command header.
pub const COMMAND_HEADER_SIZE: usize = 4;
/// The total number of bytes in a command.
pub const COMMAND_TOTAL_SIZE: usize = 64;
/// The remaining number of bytes allocated to the payload.
pub const COMMAND_PAYLOAD_SIZE: usize = COMMAND_TOTAL_SIZE - COMMAND_HEADER_SIZE;

/// A command payload.
pub type CommandPayload = [u8; COMMAND_PAYLOAD_SIZE];

/// The header for a command message.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(packed)]
pub struct CommandHeader {
    pub id: u8,
    pub status: u8,
    pub magic: u8,
    pub length: u8,
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Command {
    pub header: CommandHeader,
    pub payload: CommandPayload,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn command_size_matches_constants() {
        assert_eq!(size_of::<Command>(), COMMAND_TOTAL_SIZE);
        assert_eq!(size_of::<CommandHeader>(), COMMAND_HEADER_SIZE);
        assert_eq!(size_of::<CommandPayload>(), COMMAND_PAYLOAD_SIZE);
    }
}
