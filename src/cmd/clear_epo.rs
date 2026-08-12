use crate::traits::{Cmd, Request, Packet};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct ClearEpoCmd;

impl Packet for ClearEpoCmd {
    const PKT_TYPE: u16 = 127;
}

impl Request for ClearEpoCmd {}

impl Cmd for ClearEpoCmd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        let cmd = ClearEpoCmd;
        assert_eq!("$PMTK127*36\r\n", cmd.serialize().unwrap());
    }
}