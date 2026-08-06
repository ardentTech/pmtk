use crate::dt::datum::DatumDt;
use crate::traits::{PmtkSentence, PmtkQ, PmtkBiDir};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct DatumQ {}
impl PmtkSentence for DatumQ {
    const PKT_TYPE: u16 = 430;
}

impl PmtkBiDir for DatumQ {
    type Dt = DatumDt;
}

impl PmtkQ for DatumQ {}


#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = DatumQ {};
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: None,
            pkt_type: DatumQ::PKT_TYPE,
        };
        assert_eq!(packet, query.marshal().unwrap());
    }
}