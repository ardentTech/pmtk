use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetSbasEnabledCmd(bool);

impl SetSbasEnabledCmd {
    pub fn new(enable: bool) -> Self {
        Self(enable)
    }
}

impl Packet for SetSbasEnabledCmd {
    const PKT_TYPE: u16 = 313;
}

impl Request for SetSbasEnabledCmd {}

impl Cmd for SetSbasEnabledCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.0 as u8])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK313,1*2E\r\n", SetSbasEnabledCmd(true).serialize().unwrap());
    }
}