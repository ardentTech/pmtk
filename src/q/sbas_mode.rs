use crate::dt::sbas_mode::SbasModeDt;
use crate::traits::{PmtkSentence, PmtkQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasModeQ {}
impl PmtkSentence for SbasModeQ {
    const PKT_TYPE: u16 = 419;
}
impl PmtkQ for SbasModeQ {
    type DataType = SbasModeDt;
}


#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
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