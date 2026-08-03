use crate::response::dgps_mode::DgpsModeDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DgpsModeQuery {}
impl Message for DgpsModeQuery {
    const PKT_TYPE: u16 = 401;
}
impl Query for DgpsModeQuery {
    type R = DgpsModeDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = DgpsModeQuery {};
        let packet = PmtkPacket {
            checksum: 0x37,
            data_field: None,
            pkt_type: DgpsModeQuery::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}