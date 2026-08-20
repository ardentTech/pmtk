use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{Packet, Q, Request};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct EasyEnable {}

impl Packet for EasyEnable {
    const PKT_TYPE: u16 = 869;
}

impl Request for EasyEnable {}

impl Q for EasyEnable {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([0u8])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK869,0*29\r\n", EasyEnable {}.serialize().unwrap());
    }
}