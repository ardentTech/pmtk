use crate::dt::epo_info::EpoInfoDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct EpoInfoQ {}
impl Message for EpoInfoQ {
    const PKT_TYPE: u16 = 607;
}
impl Query for EpoInfoQ {
    type R = EpoInfoDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = EpoInfoQ {};
        let packet = PmtkPacket {
            checksum: 0x33,
            data_field: None,
            pkt_type: EpoInfoQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}