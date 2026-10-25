use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusStopLogger;

impl Packet for LocusStopLogger {
    const PKT_TYPE: PktType = [49, 56, 53]; // 185
}

impl CmdQ for LocusStopLogger {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([1u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_stop_ok() {
        assert_eq!("$PMTK185,1*23\r\n", LocusStopLogger.serialize().unwrap());
    }
}