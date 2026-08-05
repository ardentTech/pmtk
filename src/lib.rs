#![no_std]

use crate::error::PmtkError;
use crate::types::PmtkResponse;

mod parse;
pub mod error;
pub mod traits;
pub mod types;
pub mod dt;
pub mod q;
pub mod cmd;

pub struct Pmtk;

impl Pmtk {
    pub fn decode(&self, buf: &[u8]) -> Result<PmtkResponse, PmtkError> {
        PmtkResponse::try_from(buf)
    }
}