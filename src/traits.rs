use core::str::from_utf8;
use heapless::String;
use crate::error::PmtkError;
use crate::packet::{DataField, PmtkPacket};

pub trait Packet {
    const PKT_TYPE: u16;
}

pub trait Response: Packet + TryFrom<DataField> {}

// Describes PMTK sentence "request" (i.e. Cmd, Q) types and their associated "response" (i.e. Dt) types
pub trait Request: Packet {
    type R: Response;

    // TODO test
    // Tries to parse the associated response for this request.
    // can't use TryInto bc need associated type R (right? TODO?)
    fn parse_response(&self, buf: &[u8]) -> Result<Self::R, PmtkError> {
        let s = from_utf8(buf).map_err(|_| PmtkError::Parsing)?;
        if let Some(data_field) = crate::parse::packet(s)?.data_field {
            Self::R::try_from(data_field).map_err(|_| PmtkError::Parsing)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

trait Dt: Response {}

pub trait Cmd: Request {
    fn serialize(&self) -> Result<String<255>, PmtkError>;
}

pub trait Q: Request {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)?.serialize()
    }
}