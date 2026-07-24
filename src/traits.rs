use crate::error::PmtkError;
use crate::types::{DataField, PmtkPacket};

pub(crate) trait Message {
    const PKT_TYPE: u16;
}

pub(crate) trait Command<T: Response>: Message {
    const RESPONSE: Option<T>;

    fn encode(&self) -> Result<PmtkPacket, PmtkError>;
}

pub(crate) trait Query<T: Response>: Message {
    const RESPONSE: T;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)
    }
}

pub(crate) trait Response: Message + TryFrom<DataField> {}