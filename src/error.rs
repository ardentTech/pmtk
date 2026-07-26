use core::convert::Infallible;
use core::fmt;
use heapless::{CapacityError, String};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum PmtkError {
    StringCapacity,
    CoreFmt(fmt::Error),
    InvalidAckFlag(u8),
    InvalidBaudRate(u32),
    InvalidDgpsMode(u8),
    InvalidNmeaUpdateRate(u16),
    InvalidSysMsg(u8),
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