use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct WarmStartCmd;

impl PmtkSentence for WarmStartCmd {
    const PKT_TYPE: u16 = 102;
}

impl PmtkBiDir for WarmStartCmd {
    type Dt = AckDt;
}

impl PmtkCmd for WarmStartCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        PmtkPacket::new_command(Self::PKT_TYPE, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = WarmStartCmd;
        let packet = PmtkPacket {
            checksum: 0x31,
            data_field: None,
            pkt_type: WarmStartCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }
}