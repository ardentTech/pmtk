use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusConfigCmd(pub u8);

impl Packet for LocusConfigCmd {
    const PKT_TYPE: u16 = 187;
}

impl Request for LocusConfigCmd {}

impl Cmd for LocusConfigCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([1, self.0]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK187,1,5*38\r\n", LocusConfigCmd(5).serialize().unwrap());
    }
}