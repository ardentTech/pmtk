use crate::dt::dgps_mode::DgpsModeDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DgpsModeQ {}
impl Message for DgpsModeQ {
    const PKT_TYPE: u16 = 401;
}
impl Query for DgpsModeQ {
    type R = DgpsModeDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = DgpsModeQ {};
        let packet = PmtkPacket {
            checksum: 0x37,
            data_field: None,
            pkt_type: DgpsModeQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}