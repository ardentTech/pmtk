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
use crate::error::PmtkError;
use crate::error::PmtkError::Parsing;
use crate::parse;
use crate::traits::Packet;
use core::str::from_utf8;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug, PartialEq)]
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

    /// Deserializes a PMTK data type.
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let packet = parse::packet(from_utf8(buf).map_err(|_| Parsing)?)?;

        if let Some(data_field) = packet.data_field {
            match packet.pkt_type {
                // TODO reduce duplication
                AckDt::PKT_TYPE => Ok(Self::Ack(AckDt::try_from(data_field).map_err(|_| Parsing)?)),
                SysMsgDt::PKT_TYPE => Ok(Self::SysMsg(SysMsgDt::try_from(data_field).map_err(|_| Parsing)?)),
                TxtMsgDt::PKT_TYPE => Ok(Self::TxtMsg(TxtMsgDt::try_from(data_field).map_err(|_| Parsing)?)),
                DgpsModeDt::PKT_TYPE => Ok(Self::DgpsMode(DgpsModeDt::try_from(data_field).map_err(|_| Parsing)?)),
                SbasEnabledDt::PKT_TYPE => Ok(Self::SbasEnabled(SbasEnabledDt::try_from(data_field).map_err(|_| Parsing)?)),
                NmeaOutputDt::PKT_TYPE => Ok(Self::NmeaOutput(NmeaOutputDt::try_from(data_field).map_err(|_| Parsing)?)),
                SbasModeDt::PKT_TYPE => Ok(Self::SbasMode(SbasModeDt::try_from(data_field).map_err(|_| Parsing)?)),
                NavThresholdDt::PKT_TYPE => Ok(Self::NavThreshold(NavThresholdDt::try_from(data_field).map_err(|_| Parsing)?)),
                ReleaseDt::PKT_TYPE => Ok(Self::Release(ReleaseDt::try_from(data_field).map_err(|_| Parsing)?)),
                EpoInfoDt::PKT_TYPE => Ok(Self::EpoInfo(EpoInfoDt::try_from(data_field).map_err(|_| Parsing)?)),
                _ => Err(Parsing)
            }
        } else {
            Err(Parsing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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