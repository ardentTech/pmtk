use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
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
impl Packet for SetNavSpeedThresholdCmd {
    const PKT_TYPE: u16 = 397;
}

#[cfg(feature = "mt3339")]
impl Packet for SetNavSpeedThresholdCmd {
    const PKT_TYPE: u16 = 386;
}

impl Request for SetNavSpeedThresholdCmd {}

impl Cmd for SetNavSpeedThresholdCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.0])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "mt3339"))]
    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK397,0.2*3F\r\n", SetNavSpeedThresholdCmd(0.2).serialize().unwrap());
    }

    #[cfg(feature = "mt3339")]
    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK386,0.2*3F\r\n", SetNavSpeedThresholdCmd(0.2).serialize().unwrap());
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