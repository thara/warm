use std::io::BufReader;
use std::io::Read;

use thiserror::Error;

// https://webassembly.github.io/spec/core/binary/modules.html

pub struct WASMFile {}

impl WASMFile {
    fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        let mut buf = BufReader::new(bytes);

        // validate magic number
        let mut magic = [0; 4];
        buf.read_exact(&mut magic)
            .map_err(|e| ParseError::ReadFailed { source: e })?;
        if magic != [0x00, 0x61, 0x73, 0x6D] {
            return Err(ParseError::InvalidMagicNumber.into());
        }

        Ok(Self {})
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("read failed")]
    ReadFailed { source: std::io::Error },

    #[error("invalid magic number")]
    InvalidMagicNumber,
}
