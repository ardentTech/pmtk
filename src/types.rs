use crate::error::PmtkError;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub enum DgpsMode {
    None = 0x0,
    RTCM = 0x1,
    WAAS = 0x2,
}

impl TryFrom<u8> for DgpsMode {
    type Error = PmtkError;
    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(DgpsMode::None),
            1 => Ok(DgpsMode::RTCM),
            2 => Ok(DgpsMode::WAAS),
            _ => Err(PmtkError::Parsing)
        }
    }
}