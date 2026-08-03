use crate::error::PmtkError;
use crate::response::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct HotStart;

impl Message for HotStart {
    const PKT_TYPE: u16 = 101;
}

impl Command for HotStart {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = HotStart;
        let packet = PmtkPacket {
            checksum: 0x32,
            data_field: None,
            pkt_type: HotStart::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}