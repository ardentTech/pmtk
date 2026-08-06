use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ClearEpoCmd;

impl PmtkSentence for ClearEpoCmd {
    const PKT_TYPE: u16 = 127;
}

impl PmtkBiDir for ClearEpoCmd {
    type Dt = AckDt;
}

impl PmtkCmd for ClearEpoCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = ClearEpoCmd;
        let packet = PmtkPacket {
            checksum: 0x36,
            data_field: None,
            pkt_type: ClearEpoCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}