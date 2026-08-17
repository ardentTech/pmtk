use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CmdType {
    Query = 0x0,
    Set = 0x1,
    ResultForQueryOperation = 0x2
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct EasyEnableCmd { // TODO this might be better as a Response type?
    pub cmd_type: CmdType,
    pub enable: bool,
}

impl Packet for EasyEnableCmd {
    const PKT_TYPE: u16 = 869;
}

impl Request for EasyEnableCmd {}

impl Cmd for EasyEnableCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.cmd_type as u8, self.enable as u8])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK869,1,1*35\r\n", EasyEnableCmd { cmd_type: CmdType::Set, enable: true }.serialize().unwrap());
    }
}