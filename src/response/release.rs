use heapless::String;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number;
use crate::traits::{Message, Response};
use crate::types::DataField;

pub struct Release {
    pub build_id: u16,
    pub s: Option<String<16>>,
}

impl Message for Release {
    const PKT_TYPE: u16 = 705;
}

impl TryFrom<DataField> for Release {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, s) = take_until(",").parse(i)?;
        let (i, _) = char(',').parse(i)?;
        let (_, build_id) = number::<u16>(i)?;

        let s = if s.is_empty() {
            None
        } else {
            Some(String::try_from(s)?)
        };

        Ok(Release {
            build_id,
            s
        })
    }
}

impl Response for Release {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",AXN_1.3,2102,ABCD").unwrap();
        let release = Release::try_from(data_field).unwrap();
        assert_eq!(release.build_id, 2102);
        assert_eq!(release.s.unwrap(), "AXN_1.3");
    }
}