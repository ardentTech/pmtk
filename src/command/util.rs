use core::fmt::Write;
use heapless::String;
use crate::packet::DataField;

pub(crate) fn encode_data_field<const N: usize>(data: [u32; N]) -> DataField {
    let mut data_field = String::new();
    for c in data {
        write!(data_field, ",{}", c).unwrap();
    }
    data_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_data_field_ok() {
        let data = [1000u32];
        assert_eq!(",1000", encode_data_field(data));
    }
}