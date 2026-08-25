use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{Cmd, Packet, Request};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusEraseFlash {}

impl Packet for LocusEraseFlash {
    const PKT_TYPE: u16 = 184;
}

impl Request for LocusEraseFlash {}

impl Cmd for LocusEraseFlash {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([1u8])?;
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK184,1*22\r\n", LocusEraseFlash {}.serialize().unwrap());
    }
}