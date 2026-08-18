use crate::error::PmtkError;
use crate::parse;
use heapless::{String, format};

pub(crate) const PACKET_LEN: usize = 255;
pub(crate) const DATA_FIELD_LEN: usize = 242;
const PAYLOAD_LEN: usize = 246;

pub(crate) type DataField = String<DATA_FIELD_LEN>;
pub(crate) type SerializedPacket = String<PACKET_LEN>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct PmtkPacket { // TODO any way for this to be private?
    pub(crate) checksum: u8,
    pub(crate) data_field: Option<DataField>,
    pub(crate) pkt_type: u16,
}

impl PmtkPacket {
    /// Desrializes the PMTK packet.
    pub fn deserialize(raw: &str) -> Result<Self, PmtkError> {
        parse::packet(raw)
    }

    fn generate_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0, |acc, &x| acc ^ x)
    }

    pub(crate) fn new(pkt_type: u16, data_field: Option<DataField>, checksum: Option<u8>) -> Result<Self, PmtkError> {
        let payload = Self::serialize_payload(pkt_type, &data_field)?;
        let gen_checksum = Self::generate_checksum(payload.as_bytes());

        if let Some(checksum) = checksum {
            if gen_checksum != checksum { return Err(PmtkError::ChecksumMismatch); }
        }

        Ok(
            Self {
                checksum: gen_checksum,
                data_field,
                pkt_type
            }
        )
    }

    pub(crate) fn new_command(pkt_type: u16, data_field: Option<DataField>) -> Result<Self, PmtkError> {
        Self::new(pkt_type, data_field, None)
    }

    pub(crate) fn new_query(pkt_type: u16) -> Result<Self, PmtkError> {
        Self::new(pkt_type, None, None)
    }

    /// Serializes the PMTK packet.
    pub fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let payload = Self::serialize_payload(self.pkt_type, &self.data_field)?;
        format!(PACKET_LEN; "${}*{:X?}\r\n", payload, self.checksum).map_err(PmtkError::from)
    }

    fn serialize_payload(pkt_type: u16, data_field: &Option<DataField>) -> Result<String<PAYLOAD_LEN>, PmtkError> {
        if let Some(data_field) = &data_field {
            format!(PAYLOAD_LEN; "PMTK{}{}", pkt_type, data_field).map_err(PmtkError::from)
        } else {
            format!(PAYLOAD_LEN; "PMTK{}", pkt_type).map_err(PmtkError::from)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn decode_command_ok() {
        assert_eq!(
            PmtkPacket { checksum: 0x2d, data_field: Some(String::from_str(",1").unwrap()), pkt_type: 301 },
            PmtkPacket::deserialize("$PMTK301,1*2D\r\n").unwrap()
        );
    }

    #[test]
    fn decode_query_ok() {
        assert_eq!(
            PmtkPacket { checksum: 0x37, data_field: None, pkt_type: 401 },
            PmtkPacket::deserialize("$PMTK401*37\r\n").unwrap()
        );
    }


    #[test]
    fn encode_query_ok() {
        assert_eq!(
            "$PMTK401*37\r\n",
            PmtkPacket::new_query(401).unwrap().serialize().unwrap()
        );
    }

    #[test]
    fn encode_command_data_field_some_ok() {
        assert_eq!(
            "$PMTK220,1000*1F\r\n",
            PmtkPacket::new_command(220, Some(String::from_str(",1000").unwrap())).unwrap().serialize().unwrap()
        );
    }

    #[test]
    fn encode_command_data_field_none_ok() {
        assert_eq!(
            "$PMTK102*31\r\n",
            PmtkPacket::new_command(102, None).unwrap().serialize().unwrap()
        );
    }

    #[test]
    fn generate_checksum_ok() {
        assert_eq!(PmtkPacket::generate_checksum(b"PMTK011,MTKGPS"), 08);
    }
}