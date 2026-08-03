use crate::response::sbas_mode::SbasModeDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasModeQuery {}
impl Message for SbasModeQuery {
    const PKT_TYPE: u16 = 419;
}
impl Query for SbasModeQuery {
    type R = SbasModeDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = SbasModeQuery {};
        let packet = PmtkPacket {
            checksum: 0x3e,
            data_field: None,
            pkt_type: SbasModeQuery::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}