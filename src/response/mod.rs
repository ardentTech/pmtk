mod ack;
mod sys_msg;
mod txt_msg;

use crate::error::PmtkError;
use crate::packet::PmtkPacket;
use crate::response::ack::AckData;

const ACK: u16 = 1;

enum PmtkResponse {
    Ack(AckData)
}

impl TryFrom<PmtkPacket> for PmtkResponse {
    type Error = PmtkError;

    fn try_from(packet: PmtkPacket) -> Result<Self, Self::Error> {
        match packet.pkt_type {
            ACK => Ok(PmtkResponse::Ack(AckData::try_from(packet.data_field)?)),
            _ => Err(PmtkError::Parsing)
        }
    }
}