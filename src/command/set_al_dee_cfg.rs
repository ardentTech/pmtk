use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::response::ack::Ack;
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

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
pub struct SetAlDeeCfg {
    extension_gap: u32,
    extension_threshold: u32,
    snr: u8,
    sv: u8,
}

impl SetAlDeeCfg {
    pub fn new(extension_gap: u32, extension_threshold: u32, snr: u8, sv: u8) -> Result<Self, PmtkError> {
        if !(EXTENSION_GAP_MIN..=EXTENSION_GAP_MAX).contains(&extension_gap) {
            return Err(PmtkError::InputOutOfRange)
        }
        if !(EXTENSION_THRESHOLD_MIN..=EXTENSION_THRESHOLD_MAX).contains(&extension_threshold) {
            return Err(PmtkError::InputOutOfRange)
        }
        if !(SNR_MIN..=SNR_MAX).contains(&snr) {
            return Err(PmtkError::InputOutOfRange)
        }
        if !(SV_MIN..=SV_MAX).contains(&sv) {
            return Err(PmtkError::InputOutOfRange)
        }
        Ok(Self { extension_gap, extension_threshold, snr, sv })
    }
}

impl Default for SetAlDeeCfg {
    fn default() -> Self {
        Self {
            extension_gap: 60_000,
            extension_threshold: 180_000,
            snr: 30,
            sv: 1
        }
    }
}

impl Message for SetAlDeeCfg {
    const PKT_TYPE: u16 = 223;
}

impl Command for SetAlDeeCfg {
    type Response = Ack;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
        let data_field = encode_data_field([
            self.sv as u32, self.snr as u32, self.extension_threshold, self.extension_gap
        ]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::types::DataField;
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = SetAlDeeCfg::new(60_000, 180_000, 25, 1).unwrap();
        let packet = PmtkPacket {
            checksum: 0x38,
            data_field: Some(DataField::from_str(",1,25,180000,60000").unwrap()),
            pkt_type: SetAlDeeCfg::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }

    #[test]
    fn new_err() {
        assert!(SetAlDeeCfg::new(EXTENSION_GAP_MAX + 1, 180_000, 25, 1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetAlDeeCfg::new(EXTENSION_GAP_MAX, 180_000, 25, 1).is_ok());
    }
}