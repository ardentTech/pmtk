use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusStopLogger(bool);

impl LocusStopLogger {
    pub fn new(start: bool) -> Self {
        Self(!start)
    }
}

impl Packet for LocusStopLogger {
    const PKT_TYPE: u16 = 185;
}

impl CmdQ for LocusStopLogger {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_start_ok() {
        assert_eq!("$PMTK185,0*22\r\n", LocusStopLogger::new(true).serialize().unwrap());
    }

    #[test]
    fn serialize_stop_ok() {
        assert_eq!("$PMTK185,1*23\r\n", LocusStopLogger::new(false).serialize().unwrap());
    }
}