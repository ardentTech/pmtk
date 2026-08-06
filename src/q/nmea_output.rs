use crate::dt::nmea_output::NmeaOutputDt;
use crate::traits::{PmtkSentence, PmtkQ, PmtkBiDir};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NmeaOutputQ {}
impl PmtkSentence for NmeaOutputQ {
    const PKT_TYPE: u16 = 414;
}

impl PmtkBiDir for NmeaOutputQ {
    type Dt = NmeaOutputDt;
}

impl PmtkQ for NmeaOutputQ {}


#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NmeaOutputQ {};
        let packet = PmtkPacket {
            checksum: 0x33,
            data_field: None,
            pkt_type: NmeaOutputQ::PKT_TYPE,
        };
        assert_eq!(packet, query.marshal().unwrap());
    }
}