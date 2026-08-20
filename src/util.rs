use core::fmt::{Display, Write};
use heapless::String;
use crate::error::PmtkError;
use crate::packet::DataField;

pub(crate) fn encode_data_field<T: Display, const N: usize>(data: [T; N]) -> Result<DataField, PmtkError> {
    let mut data_field = String::new();
    for c in data {
        write!(data_field, ",{}", c).map_err(|_| PmtkError::Encoding)?
    }
    Ok(data_field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_data_field_ok() {
        let data = [1000u32];
        assert_eq!(",1000", encode_data_field(data).unwrap());
    }
}