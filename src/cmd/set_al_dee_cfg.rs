use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

const EXTENSION_GAP_MIN: u32 = 0;
const EXTENSION_GAP_MAX: u32 = 3_600_000;
const EXTENSION_THRESHOLD_MIN: u32 = 40_000;
const EXTENSION_THRESHOLD_MAX: u32 = 180_000;
const SNR_MIN: u8 = 25;
const SNR_MAX: u8 = 30;
const SV_MIN: u8 = 1;
const SV_MAX: u8 = 4;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetAlDeeCfgCmd {
    extension_gap: u32,
    extension_threshold: u32,
    snr: u8,
    sv: u8,
}

impl SetAlDeeCfgCmd {
    pub fn new(extension_gap: u32, extension_threshold: u32, snr: u8, sv: u8) -> Result<Self, PmtkError> {
        if !(EXTENSION_GAP_MIN..=EXTENSION_GAP_MAX).contains(&extension_gap) {
            return Err(PmtkError::OutOfRange(EXTENSION_GAP_MIN, EXTENSION_GAP_MAX, extension_gap))
        }
        if !(EXTENSION_THRESHOLD_MIN..=EXTENSION_THRESHOLD_MAX).contains(&extension_threshold) {
            return Err(PmtkError::OutOfRange(EXTENSION_THRESHOLD_MIN, EXTENSION_THRESHOLD_MAX, extension_threshold))
        }
        if !(SNR_MIN..=SNR_MAX).contains(&snr) {
            return Err(PmtkError::OutOfRange(SNR_MIN as u32, SNR_MAX as u32, snr as u32))
        }
        if !(SV_MIN..=SV_MAX).contains(&sv) {
            return Err(PmtkError::OutOfRange(SV_MIN as u32, SV_MAX as u32, sv as u32))
        }
        Ok(Self { extension_gap, extension_threshold, snr, sv })
    }
}

impl Default for SetAlDeeCfgCmd {
    fn default() -> Self {
        Self {
            extension_gap: 60_000,
            extension_threshold: 180_000,
            snr: 30,
            sv: 1
        }
    }
}

impl Packet for SetAlDeeCfgCmd {
    const PKT_TYPE: u16 = 223;
}

impl Request for SetAlDeeCfgCmd {
    type R = AckDt;
}

impl Cmd for SetAlDeeCfgCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([
            self.sv as u32, self.snr as u32, self.extension_threshold, self.extension_gap
        ]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::packet::DataField;
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK223,1,25,180000,60000*38\r\n", SetAlDeeCfgCmd::new(60_000, 180_000, 25, 1).unwrap().serialize().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetAlDeeCfgCmd::new(EXTENSION_GAP_MAX + 1, 180_000, 25, 1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetAlDeeCfgCmd::new(EXTENSION_GAP_MAX, 180_000, 25, 1).is_ok());
    }
}