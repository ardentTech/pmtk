use crate::dt::dgps_mode::DgpsModeDt;
use crate::traits::{PmtkSentence, PmtkQ, PmtkBiDir};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DgpsModeQ {}
impl PmtkSentence for DgpsModeQ {
    const PKT_TYPE: u16 = 401;
}

impl PmtkBiDir for DgpsModeQ {
    type Dt = DgpsModeDt;
}

impl PmtkQ for DgpsModeQ {}


#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = DgpsModeQ {};
        let packet = PmtkPacket {
            checksum: 0x37,
            data_field: None,
            pkt_type: DgpsModeQ::PKT_TYPE,
        };
        assert_eq!(packet, query.marshal().unwrap());
    }
}