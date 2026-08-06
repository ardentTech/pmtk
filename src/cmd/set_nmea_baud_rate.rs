use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaBaudRateCmd(u32);

impl SetNmeaBaudRateCmd {
    pub fn new(rate: u32) -> Result<Self, PmtkError> {
        if ![0, 4800, 9600, 14400, 19200, 38400, 57600, 115200].contains(&rate) {
            return Err(PmtkError::InvalidChoice(rate));
        }
        Ok(Self(rate))
    }
}

impl PmtkSentence for SetNmeaBaudRateCmd {
    const PKT_TYPE: u16 = 251;
}

impl PmtkBiDir for SetNmeaBaudRateCmd {
    type Dt = AckDt;
}

impl PmtkCmd for SetNmeaBaudRateCmd {
    fn marshal(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([self.0]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::packet::DataField;
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = SetNmeaBaudRateCmd(38400);
        let packet = PmtkPacket {
            checksum: 0x27,
            data_field: Some(DataField::from_str(",38400").unwrap()),
            pkt_type: SetNmeaBaudRateCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaBaudRateCmd::new(1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaBaudRateCmd::new(38400).is_ok());
    }
}