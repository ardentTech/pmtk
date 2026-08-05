use crate::dt::epo_info::EpoInfoDt;
use crate::traits::{PmtkSentence, PmtkQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct EpoInfoQ {}
impl PmtkSentence for EpoInfoQ {
    const PKT_TYPE: u16 = 607;
}
impl PmtkQ for EpoInfoQ {
    type DataType = EpoInfoDt;
}


#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
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