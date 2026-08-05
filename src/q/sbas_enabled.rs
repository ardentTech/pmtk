use crate::dt::sbas_enabled;
use crate::traits::{PmtkSentence, PmtkQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasEnabledQ {}
impl PmtkSentence for SbasEnabledQ {
    const PKT_TYPE: u16 = 413;
}
impl PmtkQ for SbasEnabledQ {
    type DataType = sbas_enabled::SbasEnabledDt;
}

#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = SbasEnabledQ {};
        let packet = PmtkPacket {
            checksum: 0x34,
            data_field: None,
            pkt_type: SbasEnabledQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}