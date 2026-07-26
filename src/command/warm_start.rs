use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct WarmStart;

impl Message for WarmStart {
    const PKT_TYPE: u16 = 102;
}

impl Command for WarmStart {
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
        let cmd = WarmStart;
        let packet = PmtkPacket {
            checksum: 0x31,
            data_field: None,
            pkt_type: WarmStart::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}