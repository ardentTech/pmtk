use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{Packet, CmdQ};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusDataQ(bool);

impl LocusDataQ {
    pub fn new(partial: bool) -> Self {
        Self(partial)
    }
}

impl Packet for LocusDataQ {
    const PKT_TYPE: PktType = [54, 50, 50]; // 622
}

impl CmdQ for LocusDataQ {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_full_ok() {
        assert_eq!("$PMTK622,0*28\r\n", LocusDataQ::new(false).serialize().unwrap());
    }

    #[test]
    fn serialize_partial_ok() {
        assert_eq!("$PMTK622,1*29\r\n", LocusDataQ::new(true).serialize().unwrap());
    }
}