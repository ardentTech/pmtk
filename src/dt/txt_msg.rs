use nom::character::complete::char;
use crate::error::PmtkError;
use crate::traits::{Packet, Response};
use crate::packet::{DataField, DATA_FIELD_LEN};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug, PartialEq)]
pub struct TxtMsgDt(pub [u8; DATA_FIELD_LEN]);

impl Packet for TxtMsgDt {
    const PKT_TYPE: u16 = 11;
}

impl TryFrom<DataField> for TxtMsgDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (msg, _) = char(',')(i)?;

        let mut buf = [0u8; DATA_FIELD_LEN];
        msg.as_bytes().iter().enumerate().for_each(|(i, b)| buf[i] = *b);
        Ok(TxtMsgDt(buf))
    }
}

impl Response for TxtMsgDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use nom::AsBytes;
    use super::*;
    use crate::packet::DataField;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",MTKGPS").unwrap();
        let txt_msg = TxtMsgDt::try_from(data_field).unwrap();
        assert_eq!(txt_msg.0, [77, 84, 75, 71, 80, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}