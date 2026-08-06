use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ColdStartCmd;

impl PmtkSentence for ColdStartCmd {
    const PKT_TYPE: u16 = 103;
}

impl PmtkBiDir for ColdStartCmd {
    type Dt = AckDt;
}

impl PmtkCmd for ColdStartCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = ColdStartCmd;
        let packet = PmtkPacket {
            checksum: 0x30,
            data_field: None,
            pkt_type: ColdStartCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}