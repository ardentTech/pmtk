use crate::dt::release::ReleaseDt;
use crate::traits::{PmtkSentence, PmtkQ, PmtkBiDir};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct ReleaseQ {}

impl PmtkSentence for ReleaseQ {
    const PKT_TYPE: u16 = 605;
}

impl PmtkBiDir for ReleaseQ {
    type Dt = ReleaseDt;
}

impl PmtkQ for ReleaseQ {}

#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = ReleaseQ {};
        let packet = PmtkPacket {
            checksum: 0x31,
            data_field: None,
            pkt_type: ReleaseQ::PKT_TYPE,
        };
        assert_eq!(packet, query.marshal().unwrap());
    }
}