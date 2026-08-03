use core::str::from_utf8;
use heapless::{format, String};
use crate::error::PmtkError;
use crate::error::PmtkError::Decoding;
use crate::parse;
use crate::dt::ack::AckDt;
use crate::dt::dgps_mode::DgpsModeDt;
use crate::dt::epo_info::EpoInfoDt;
use crate::dt::nav_threshold::NavThresholdDt;
use crate::dt::nmea_output::NmeaOutputDt;
use crate::dt::release::ReleaseDt;
use crate::dt::sbas_enabled::SbasEnabledDt;
use crate::dt::sbas_mode::SbasModeDt;
use crate::dt::sys_msg::SysMsgDt;
use crate::dt::txt_msg::TxtMsgDt;
use crate::traits::Message;

const PACKET_LEN: usize = 255;
pub(crate) const DATA_FIELD_LEN: usize = 242;
const PAYLOAD_LEN: usize = 246;

pub(crate) type DataField = String<DATA_FIELD_LEN>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct PmtkPacket {
    pub(crate) checksum: u8,
    pub(crate) data_field: Option<DataField>, // TODO diff between Req (cmd + q) and Res?
    pub(crate) pkt_type: u16
}

impl PmtkPacket {
    pub fn decode(raw: &str) -> Result<Self, PmtkError> {
        parse::packet(raw)
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

    fn payload(pkt_type: u16, data_field: &Option<DataField>) -> Result<String<PAYLOAD_LEN>, PmtkError> {
        if let Some(data_field) = &data_field {
            format!(PAYLOAD_LEN; "PMTK{}{}", pkt_type, data_field).map_err(PmtkError::from)
        } else {
            format!(PAYLOAD_LEN; "PMTK{}", pkt_type).map_err(PmtkError::from)
        }
    }

    pub(crate) fn new(pkt_type: u16, data_field: Option<DataField>, checksum: Option<u8>) -> Result<Self, PmtkError> {
        let payload = Self::payload(pkt_type, &data_field)?;
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
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub enum PmtkResponse {
    Ack(AckDt),
    SysMsg(SysMsgDt),
    TxtMsg(TxtMsgDt),
    DgpsMode(DgpsModeDt),
    SbasEnabled(SbasEnabledDt),
    NmeaOutput(NmeaOutputDt),
    SbasMode(SbasModeDt),
    NavThreshold(NavThresholdDt),
    Release(ReleaseDt),
    EpoInfo(EpoInfoDt),
}
impl TryFrom<&[u8]> for PmtkResponse {
    type Error = PmtkError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let packet = parse::packet(from_utf8(buf).map_err(|_| PmtkError::Parsing)?)?;

        if let Some(data_field) = packet.data_field {
            match packet.pkt_type {
                AckDt::PKT_TYPE => Ok(Self::Ack(AckDt::try_from(data_field).map_err(|_| Decoding)?)),
                SysMsgDt::PKT_TYPE => Ok(Self::SysMsg(SysMsgDt::try_from(data_field).map_err(|_| Decoding)?)),
                TxtMsgDt::PKT_TYPE => Ok(Self::TxtMsg(TxtMsgDt::try_from(data_field).map_err(|_| Decoding)?)),
                DgpsModeDt::PKT_TYPE => Ok(Self::DgpsMode(DgpsModeDt::try_from(data_field).map_err(|_| Decoding)?)),
                SbasEnabledDt::PKT_TYPE => Ok(Self::SbasEnabled(SbasEnabledDt::try_from(data_field).map_err(|_| Decoding)?)),
                NmeaOutputDt::PKT_TYPE => Ok(Self::NmeaOutput(NmeaOutputDt::try_from(data_field).map_err(|_| Decoding)?)),
                SbasModeDt::PKT_TYPE => Ok(Self::SbasMode(SbasModeDt::try_from(data_field).map_err(|_| Decoding)?)),
                NavThresholdDt::PKT_TYPE => Ok(Self::NavThreshold(NavThresholdDt::try_from(data_field).map_err(|_| Decoding)?)),
                ReleaseDt::PKT_TYPE => Ok(Self::Release(ReleaseDt::try_from(data_field).map_err(|_| Decoding)?)),
                EpoInfoDt::PKT_TYPE => Ok(Self::EpoInfo(EpoInfoDt::try_from(data_field).map_err(|_| Decoding)?)),
                _ => Err(Decoding)
            }
        } else {
            Err(Decoding)
        }
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

    #[test]
    fn pmtk_response_try_from_checksum_err() {
        let raw = "$PMTK514,0,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0,0*3B\r\n";
        assert!(PmtkResponse::try_from(raw.as_bytes()).is_err());
    }

    #[test]
    fn pmtk_response_try_from_parse_err() {
        let raw = "$PMTK514,0,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0*2B\r\n";
        assert!(PmtkResponse::try_from(raw.as_bytes()).is_err());
    }

    #[test]
    fn pmtk_response_try_from_ok() {
        let raw = "$PMTK514,0,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0,0*2B\r\n";
        assert!(PmtkResponse::try_from(raw.as_bytes()).is_ok());
    }
}