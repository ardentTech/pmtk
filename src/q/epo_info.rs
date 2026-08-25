use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct EpoInfoQ {}
impl Packet for EpoInfoQ {
    const PKT_TYPE: u16 = 607;
}

impl CmdQ for EpoInfoQ {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK607*33\r\n", EpoInfoQ {}.serialize().unwrap());
    }
}