use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ColdStart;

impl Message for ColdStart {
    const PKT_TYPE: u16 = 103;
}

impl Command for ColdStart {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = ColdStart;
        let packet = PmtkPacket {
            checksum: 0x30,
            data_field: None,
            pkt_type: ColdStart::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}