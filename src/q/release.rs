use crate::dt::release;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct ReleaseQ {}

impl Message for ReleaseQ {
    const PKT_TYPE: u16 = 605;
}

impl Query for ReleaseQ {
    type R = release::ReleaseDt;
}

#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = ReleaseQ {};
        let packet = PmtkPacket {
            checksum: 0x31,
            data_field: None,
            pkt_type: ReleaseQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}