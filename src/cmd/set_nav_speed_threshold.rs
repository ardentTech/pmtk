use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{PmtkCmd, PmtkBiDir, PmtkSentence};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNavSpeedThresholdCmd(f32);

impl SetNavSpeedThresholdCmd {
    pub fn new(threshold: f32) -> Result<Self, PmtkError> {
        if ![0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.5, 2.0].contains(&threshold) {
            return Err(PmtkError::InvalidNavSpeedThreshold(threshold));
        }
        Ok(Self(threshold))
    }
}

#[cfg(not(feature = "mt3339"))]
impl PmtkSentence for SetNavSpeedThresholdCmd {
    const PKT_TYPE: u16 = 397;
}

#[cfg(feature = "mt3339")]
impl PmtkSentence for SetNavSpeedThresholdCmd {
    const PKT_TYPE: u16 = 386;
}

impl PmtkBiDir for SetNavSpeedThresholdCmd {
    type Dt = AckDt;
}

impl PmtkCmd for SetNavSpeedThresholdCmd {
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
        let cmd = SetNavSpeedThresholdCmd(0.2);
        let packet = PmtkPacket {
            checksum: 0x3f,
            data_field: Some(DataField::from_str(",0.2").unwrap()),
            pkt_type: SetNavSpeedThresholdCmd::PKT_TYPE,
        };
        assert_eq!(packet, cmd.marshal().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNavSpeedThresholdCmd::new(0.5).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNavSpeedThresholdCmd::new(0.2).is_ok());
    }
}