#[derive(Debug)]
pub enum PmtkError {
    Parsing
}
impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for PmtkError {
    fn from(_error: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::Parsing
    }
}