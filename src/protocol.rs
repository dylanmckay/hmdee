//! The low-level protocol types.

/// The size of the command header.
pub const COMMAND_HEADER_SIZE: usize = 4;

/// The header for a command message.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(packed)]
pub struct CommandHeader {
    pub id: u8,
    pub status: u8,
    pub magic: u8,
    pub length: u8,
}

#[derive(Clone)]
#[repr(packed)]
pub struct Command {
    pub header: CommandHeader,
    pub payload: Vec<u8>,
}

impl CommandHeader {
    pub fn raw_bytes(&self) -> Vec<u8> {
        vec![self.id, self.status, self.magic, self.length]
    }
}

impl Command {
    pub fn raw_bytes(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();

        raw_bytes.extend(self.header.raw_bytes());
        raw_bytes.extend(self.payload.iter());

        raw_bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn command_header_size_matches_constant() {
        assert_eq!(size_of::<CommandHeader>(), COMMAND_HEADER_SIZE);
    }

    #[test]
    fn command_raw_bytes_is_correct() {
        let command = Command {
            header: CommandHeader {
                id: 0x69,
                status: 123,
                magic: 88,
                length: 5,
            },
            payload: vec![5,4,3,2,1],
        };
        assert_eq!(&[0x69, 123, 88, 5, 5, 4, 3, 2, 1], &command.raw_bytes()[..]);
    }


}
