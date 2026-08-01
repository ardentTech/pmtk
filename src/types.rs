use heapless::{format, String};
use crate::error::PmtkError;
use crate::parse::packet;

const PACKET_LEN: usize = 255;
pub(crate) const DATA_FIELD_LEN: usize = 242;
const PAYLOAD_LEN: usize = 246;

pub(crate) type DataField = String<DATA_FIELD_LEN>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct PmtkPacket {
    pub(crate) checksum: u8,
    pub(crate) data_field: Option<DataField>,
    pub(crate) pkt_type: u16
}

impl PmtkPacket {
    pub fn decode(raw: &str) -> Result<Self, PmtkError> {
        packet(raw)
    }

    pub fn encode(&self) -> Result<String<PACKET_LEN>, PmtkError> {
        Ok(if let Some(data_field) = &self.data_field {
            format!(PACKET_LEN; "$PMTK{}{}*{:X?}\r\n", self.pkt_type, data_field, self.checksum)?
        } else {
            format!(PACKET_LEN; "$PMTK{}*{:X?}\r\n", self.pkt_type, self.checksum)?
        })
    }

    fn generate_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0, |acc, &x| acc ^ x)
    }

    pub(crate) fn new_command(pkt_type: u16, data_field: Option<DataField>) -> Result<Self, PmtkError> {
        let payload: String<PAYLOAD_LEN> = if let Some(data_field) = &data_field {
            format!(PAYLOAD_LEN; "PMTK{}{}", pkt_type, data_field)?
        } else {
            format!(PAYLOAD_LEN; "PMTK{}", pkt_type)?
        };
        let checksum = Self::generate_checksum(payload.as_bytes());
        Ok(Self {
            checksum,
            data_field,
            pkt_type
        })
    }

    pub(crate) fn new_query(pkt_type: u16) -> Result<Self, PmtkError> {
        let payload = format!(PAYLOAD_LEN; "PMTK{}", pkt_type)?;
        let checksum = Self::generate_checksum(payload.as_bytes());
        Ok(Self {
            checksum,
            data_field: None,
            pkt_type
        })
    }
}


#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn decode_command_ok() {
        assert_eq!(
            PmtkPacket { checksum: 0x2d, data_field: Some(String::from_str(",1").unwrap()), pkt_type: 301 },
            PmtkPacket::decode("$PMTK301,1*2D\r\n").unwrap()
        );
    }

    #[test]
    fn decode_query_ok() {
        assert_eq!(
            PmtkPacket { checksum: 0x37, data_field: None, pkt_type: 401 },
            PmtkPacket::decode("$PMTK401*37\r\n").unwrap()
        );
    }


    #[test]
    fn encode_query_ok() {
        assert_eq!(
            "$PMTK401*37\r\n",
            PmtkPacket::new_query(401).unwrap().encode().unwrap()
        );
    }

    #[test]
    fn encode_command_data_field_some_ok() {
        assert_eq!(
            "$PMTK220,1000*1F\r\n",
            PmtkPacket::new_command(220, Some(String::from_str(",1000").unwrap())).unwrap().encode().unwrap()
        );
    }

    #[test]
    fn encode_command_data_field_none_ok() {
        assert_eq!(
            "$PMTK102*31\r\n",
            PmtkPacket::new_command(102, None).unwrap().encode().unwrap()
        );
    }

    #[test]
    fn generate_checksum_ok() {
        assert_eq!(PmtkPacket::generate_checksum(b"PMTK011,MTKGPS"), 08);
    }
}