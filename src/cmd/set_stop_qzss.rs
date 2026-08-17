use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetStopQzssCmd(pub bool);

impl Packet for SetStopQzssCmd {
    const PKT_TYPE: u16 = 352;
}

impl Request for SetStopQzssCmd {}

impl Cmd for SetStopQzssCmd {
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
        assert_eq!("$PMTK352,1*2B\r\n", SetStopQzssCmd(true).serialize().unwrap());
    }
}