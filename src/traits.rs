use crate::error::PmtkError;
use crate::types::{DataField, PmtkPacket};

pub trait Message {
    const PKT_TYPE: u16;
}

pub trait Command: Message {
    type Response: Response;

    fn encode(&self) -> Result<PmtkPacket, PmtkError>;
}

pub trait Query: Message {
    type Response: Response;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}

pub trait Response: Message + TryFrom<DataField> {}