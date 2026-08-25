use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{Cmd, Packet, Request};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusConfigCmd(u8);

impl LocusConfigCmd {
    pub fn new(interval: u8) -> Self {
        Self(interval)
    }
}

impl Packet for LocusConfigCmd {
    const PKT_TYPE: u16 = 187;
}

impl Request for LocusConfigCmd {}

impl Cmd for LocusConfigCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([1, self.0])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
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