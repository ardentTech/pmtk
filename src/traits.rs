use core::str::from_utf8;
use crate::error::PmtkError;
use crate::packet::{DataField, PmtkPacket};

pub trait PmtkSentence {
    const PKT_TYPE: u16;
}

pub trait PmtkDt: PmtkSentence + TryFrom<DataField> {}

// Describes PMTK sentence "request" (i.e. Cmd, Q) types and their associated "response" (i.e. Dt) types
pub trait PmtkBiDir {
    type Dt: PmtkDt;

    // TODO test
    fn parse_dt(&self, buf: &[u8]) -> Result<Self::Dt, PmtkError> {
        let s = from_utf8(buf).map_err(|_| PmtkError::Parsing)?;
        let packet = crate::parse::packet(s)?;
        if let Some(data_field) = packet.data_field {
            Self::Dt::try_from(data_field).map_err(|_| PmtkError::Parsing)
        } else {
            Err(PmtkError::Parsing)
        }
    }
}

pub trait PmtkCmd: PmtkSentence + PmtkBiDir {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError>;
}

pub trait PmtkQ: PmtkSentence + PmtkBiDir {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}