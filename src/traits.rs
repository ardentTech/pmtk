use core::str::from_utf8;
use crate::error::PmtkError;
use crate::packet::{DataField, PmtkPacket};

pub trait PmtkSentence {
    const PKT_TYPE: u16;
}

pub trait PmtkDt: PmtkSentence + TryFrom<DataField> {}

pub trait PmtkCmd: PmtkSentence {
    type DataType: PmtkDt;

    // TODO is this even necessary? can use in higher-level lib at all?
    // TODO test
    // TODO nail down error types
    fn decode(&self, buf: &[u8]) -> Result<Self::DataType, PmtkError> {
        let raw = from_utf8(buf).map_err(|_| PmtkError::Parsing)?;
        let packet = crate::parse::packet(raw)?;
        if let Some(data_field) = packet.data_field {
            Self::DataType::try_from(data_field).map_err(|_| PmtkError::Decoding)
        } else {
            Err(PmtkError::Decoding)
        }
    }

    fn encode(&self) -> Result<PmtkPacket, PmtkError>;
}

pub trait PmtkQ: PmtkSentence {
    type DataType: PmtkDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}