use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusLogNow {}

impl Packet for LocusLogNow {
    const PKT_TYPE: PktType = [49, 56, 54]; // 186
}

impl CmdQ for LocusLogNow {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([1u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK186,1*20\r\n", LocusLogNow {}.serialize().unwrap());
    }
}