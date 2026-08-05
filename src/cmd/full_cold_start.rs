use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct FullColdStartCmd;

impl PmtkSentence for FullColdStartCmd {
    const PKT_TYPE: u16 = 104;
}

impl PmtkCmd for FullColdStartCmd {
    type DataType = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = FullColdStartCmd;
        let packet = PmtkPacket {
            checksum: 0x37,
            data_field: None,
            pkt_type: FullColdStartCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}