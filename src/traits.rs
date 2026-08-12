use crate::error::PmtkError;
use crate::packet::{DataField, PmtkPacket, SerializedPacket};

pub trait Packet {
    const PKT_TYPE: u16;
}

pub trait Response: Packet + TryFrom<DataField> {}

pub trait Request: Packet {}

pub trait Dt: Response {}

pub trait Cmd: Request {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)?.serialize()
    }
}

pub trait Q: Request {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        PmtkPacket::new_query(Self::PKT_TYPE)?.serialize()
    }
}