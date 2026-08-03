use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct FullColdStart;

impl Message for FullColdStart {
    const PKT_TYPE: u16 = 104;
}

impl Command for FullColdStart {
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
        let cmd = FullColdStart;
        let packet = PmtkPacket {
            checksum: 0x37,
            data_field: None,
            pkt_type: FullColdStart::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}