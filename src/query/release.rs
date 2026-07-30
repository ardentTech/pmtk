use crate::response::release;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct Release {}

impl Message for Release {
    const PKT_TYPE: u16 = 605;
}

impl Query for Release {
    type Response = release::Release;
}

#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = Release {};
        let packet = PmtkPacket {
            checksum: 0x31,
            data_field: None,
            pkt_type: Release::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}