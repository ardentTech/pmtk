use crate::error::PmtkError;
use crate::packet::{DataField, PktType, PmtkPacket, SerializedPacket};

pub trait Packet {
    /// Uniquely identifies a PMTK packet.
    const PKT_TYPE: PktType;
}

pub trait Dt: Packet + TryFrom<DataField> {}

pub trait CmdQ: Packet {
    /// Serializes a PMTK command.
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        PmtkPacket::new_request(Self::PKT_TYPE, None)?.serialize()
    }
}