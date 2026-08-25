use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SbasEnabledQ {}

impl Packet for SbasEnabledQ {
    const PKT_TYPE: u16 = 413;
}
impl CmdQ for SbasEnabledQ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK413*34\r\n", SbasEnabledQ {}.serialize().unwrap());
    }
}