use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ClearEpo;

impl Message for ClearEpo {
    const PKT_TYPE: u16 = 127;
}

impl Command for ClearEpo {
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
        let cmd = ClearEpo;
        let packet = PmtkPacket {
            checksum: 0x36,
            data_field: None,
            pkt_type: ClearEpo::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}