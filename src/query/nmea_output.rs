use crate::response::nmea_output::NmeaOutput;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NmeaOutputQuery {}
impl Message for NmeaOutputQuery {
    const PKT_TYPE: u16 = 414;
}
impl Query for NmeaOutputQuery {
    type Response = NmeaOutput;
}


#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NmeaOutputQuery {};
        let packet = PmtkPacket {
            checksum: 0x33,
            data_field: None,
            pkt_type: NmeaOutputQuery::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}