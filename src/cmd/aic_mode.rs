use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct AicModeCmd(pub bool);

impl Packet for AicModeCmd {
    const PKT_TYPE: u16 = 286;
}

impl Request for AicModeCmd {}

impl Cmd for AicModeCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([self.0 as u8]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK286,1*23\r\n", AicModeCmd(true).serialize().unwrap());
    }
}