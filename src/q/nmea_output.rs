use crate::dt::nmea_output::NmeaOutputDt;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NmeaOutputQ {}
impl Message for NmeaOutputQ {
    const PKT_TYPE: u16 = 414;
}
impl Query for NmeaOutputQ {
    type R = NmeaOutputDt;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NmeaOutputQ {};
        let packet = PmtkPacket {
            checksum: 0x33,
            data_field: None,
            pkt_type: NmeaOutputQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}