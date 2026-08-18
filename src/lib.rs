#![no_std]

use core::str::from_utf8;
use crate::error::PmtkError;
use crate::error::PmtkError::Parsing;
use crate::packet::PmtkPacket;

mod parse;
pub mod error;
pub mod traits;
pub mod packet;
pub mod dt;
pub mod q;
pub mod cmd;

pub struct Pmtk {}

impl Pmtk {
    /// Parses a raw byte array into a PMTK packet.
    pub fn parse(buf: &[u8]) -> Result<PmtkPacket, PmtkError> {
        parse::packet(from_utf8(buf).map_err(|_| Parsing)?)
    }
}