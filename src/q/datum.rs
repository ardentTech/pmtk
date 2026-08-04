use crate::dt::datum::DatumDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DatumQuery {}
impl Message for DatumQuery {
    const PKT_TYPE: u16 = 430;
}
impl Query for DatumQuery {
    type R = DatumDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = DatumQuery {};
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: None,
            pkt_type: DatumQuery::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}