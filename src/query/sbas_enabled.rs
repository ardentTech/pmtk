use crate::response::sbas_enabled;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasEnabled {}
impl Message for SbasEnabled {
    const PKT_TYPE: u16 = 413;
}
impl Query for SbasEnabled {
    type R = sbas_enabled::SbasEnabledDt;
}

#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = SbasEnabled {};
        let packet = PmtkPacket {
            checksum: 0x34,
            data_field: None,
            pkt_type: SbasEnabled::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}