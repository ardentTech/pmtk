use core::convert::Infallible;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum PmtkError {
    InvalidBaudRate(u32),
    InvalidNmeaUpdateRate(u32),
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