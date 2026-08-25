use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

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

impl CmdQ for SetNmeaBaudRateCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
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