use crate::traits::{Packet, Request, Q};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusStatus {}

impl Packet for LocusStatus {
    const PKT_TYPE: u16 = 183;
}

impl Request for LocusStatus {}

impl Q for LocusStatus {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK183*38\r\n", LocusStatus {}.serialize().unwrap());
    }
}