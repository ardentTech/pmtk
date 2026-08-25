use crate::error::PmtkError;
use crate::packet::{PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

const MIN_MS: u16 = 100;
const MAX_MS: u16 = 10_000;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaUpdateRateCmd(u16);

impl SetNmeaUpdateRateCmd {
    pub fn new(ms: u16) -> Result<SetNmeaUpdateRateCmd, PmtkError> {
        if ms < MIN_MS || ms > MAX_MS {
            Err(PmtkError::OutOfRange(MIN_MS as u32, MAX_MS as u32, ms as u32))
        } else {
            Ok(SetNmeaUpdateRateCmd(ms))
        }
    }
}

impl Packet for SetNmeaUpdateRateCmd {
    const PKT_TYPE: u16 = 220;
}

impl CmdQ for SetNmeaUpdateRateCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([self.0 as u32])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK220,1000*1F\r\n", SetNmeaUpdateRateCmd(1000).serialize().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaUpdateRateCmd::new(MAX_MS + 1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaUpdateRateCmd::new(MIN_MS).is_ok());
    }
}