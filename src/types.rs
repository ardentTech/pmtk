use heapless::{format, String};
use crate::error::PmtkError;

const DATA_FIELD_LEN: usize = 242;
const PAYLOAD_LEN: usize = 246;

pub(crate) type DataField = String<DATA_FIELD_LEN>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub(crate) struct PmtkPacket {
    pub(crate) checksum: u8,
    pub(crate) data_field: Option<DataField>,
    pub(crate) pkt_type: u16
}

impl PmtkPacket {
    pub(crate) fn new_command(pkt_type: u16, data_field: DataField) -> Result<Self, PmtkError> {
        let payload = format!(PAYLOAD_LEN; "PMTK{}{}", pkt_type, data_field).map_err(|e| PmtkError::CoreFmt(e))?;
        let checksum = Self::generate_checksum(payload.as_bytes());
        Ok(Self {
            checksum,
            data_field: Some(data_field),
            pkt_type
        })
    }

    pub(crate) fn new_query(pkt_type: u16) -> Result<Self, PmtkError> {
        let payload = format!(PAYLOAD_LEN; "PMTK{}", pkt_type).map_err(|e| PmtkError::CoreFmt(e))?;
        let checksum = Self::generate_checksum(payload.as_bytes());
        Ok(Self {
            checksum,
            data_field: None,
            pkt_type
        })
    }

    fn generate_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0, |acc, &x| acc ^ x)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn decode_ok() {
    //     assert_eq!(
    //         PmtkPacket { checksum: 31, data_field: Some(String::from_str(",1000").unwrap()), pkt_type: 220 },
    //         PmtkPacket::decode("$PMTK220,1000*1F\r\n").unwrap()
    //     );
    // }
    //
    // #[test]
    // fn encode_ok() {
    //     assert_eq!(
    //         "$PMTK220,1000*1F\r\n",
    //         PmtkPacket::new(String::from_str(",1000").unwrap(), 220).encode()
    //     );
    // }

    #[test]
    fn generate_checksum_ok() {
        assert_eq!(PmtkPacket::generate_checksum(b"PMTK011,MTKGPS"), 08);
    }
}