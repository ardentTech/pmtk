use crate::error::PmtkError;
use crate::types::{DataField, PmtkPacket};

pub(crate) trait Message {
    const PKT_TYPE: u16;
}

pub(crate) trait Command: Message {
    type Response: Response;

    fn encode(&self) -> Result<PmtkPacket, PmtkError>;
}

pub(crate) trait Query: Message {
    type Response: Response;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}

pub(crate) trait Response: Message + TryFrom<DataField> {}