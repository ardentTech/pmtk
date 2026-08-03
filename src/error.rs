use core::convert::Infallible;
use core::fmt;
use heapless::CapacityError;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum PmtkError {
    StringCapacity,
    ChecksumMismatch,
    CoreFmt(fmt::Error),
    Decoding,
    InvalidAckFlag(u8),
    InvalidBaudRate(u32),
    InvalidDgpsMode(u8),
    InvalidNavSpeedThreshold(f32),
    InvalidNmeaOutputFrequency(u8),
    InvalidNmeaUpdateRate(u16),
    InvalidPeriodModeRunTime(u32),
    InvalidPeriodModeSleepTime(u32),
    InvalidPeriodModeSecondRunTime(u32),
    InvalidPeriodModeSecondSleepTime(u32),
    InvalidSbasMode(u8),
    InvalidSysMsg(u8),
    InputOutOfRange, // TODO use this and replace above
    Parsing
}
impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for PmtkError {
    fn from(_error: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::Parsing
    }
}

// needed to do this to return PmtkError in TryFrom impls
impl From<Infallible> for PmtkError {
    fn from(_error: Infallible) -> Self {
        unreachable!()
    }
}

impl From<CapacityError> for PmtkError {
    fn from(_error: CapacityError) -> Self {
        Self::StringCapacity
    }
}

impl From<fmt::Error> for PmtkError {
    fn from(_error: fmt::Error) -> Self {
        Self::CoreFmt(_error)
    }
}