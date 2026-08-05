use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasModeQ {}
impl Message for SbasModeQ {
    const PKT_TYPE: u16 = 419;
}
impl Query for SbasModeQ {
    type R = SbasModeDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = SbasModeQ {};
        let packet = PmtkPacket {
            checksum: 0x3e,
            data_field: None,
            pkt_type: SbasModeQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}