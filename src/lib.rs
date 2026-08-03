#![no_std]

use crate::error::PmtkError;
use crate::types::PmtkResponse;

mod parse;
pub mod error;
pub mod traits;
pub mod types;
pub mod response;
pub mod query;
pub mod command;

struct Pmtk;

impl Pmtk {
    fn decode(&self, buf: &[u8]) -> Result<PmtkResponse, PmtkError> {
        PmtkResponse::try_from(buf)
    }
}