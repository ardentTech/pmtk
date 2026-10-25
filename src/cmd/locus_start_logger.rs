// This isn't technically a PMTK A11 command, but I found using `LocusStopLogger` (which is the
// documented API) to both start and stop the logger clunky and unintuitive.

use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusStartLogger;

impl Packet for LocusStartLogger {
    const PKT_TYPE: PktType = [49, 56, 53]; // 185
}

impl CmdQ for LocusStartLogger {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([0u8])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_start_ok() {
        assert_eq!("$PMTK185,0*22\r\n", LocusStartLogger.serialize().unwrap());
    }
}