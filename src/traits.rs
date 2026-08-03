use core::str::from_utf8;
use crate::error::PmtkError;
use crate::types::{DataField, PmtkPacket};

pub trait Message {
    const PKT_TYPE: u16;
}

pub trait Command: Message {
    type R: Response;

    // TODO test
    // TODO nail down error types
    fn decode(&self, buf: &[u8]) -> Result<Self::R, PmtkError> {
        let raw = from_utf8(buf).map_err(|_| PmtkError::Parsing)?;
        let packet = crate::parse::packet(raw)?;
        if let Some(data_field) = packet.data_field {
            Self::R::try_from(data_field).map_err(|_| PmtkError::Decoding)
        } else {
            Err(PmtkError::Decoding)
        }
    }

    fn encode(&self) -> Result<PmtkPacket, PmtkError>;
}

pub trait Query: Message {
    type R: Response;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}

pub trait Response: Message + TryFrom<DataField> {}