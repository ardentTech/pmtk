use core::convert::Infallible;
use core::fmt;
use heapless::CapacityError;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PmtkError {
    ChecksumMismatch,
    CoreFmt(fmt::Error),
    Encoding,
    InvalidChoice(u32),
    InvalidNavSpeedThreshold(f32),
    OutOfRange(u32, u32, u32),
    Parsing,
    StringCapacity,
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