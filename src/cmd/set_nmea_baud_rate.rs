use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
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

impl Packet for SetNmeaBaudRateCmd {
    const PKT_TYPE: u16 = 251;
}

impl Request for SetNmeaBaudRateCmd {}

impl Cmd for SetNmeaBaudRateCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.0])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK251,38400*27\r\n", SetNmeaBaudRateCmd(38400).serialize().unwrap());
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