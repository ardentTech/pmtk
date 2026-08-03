use nom::character::complete::char;
use nom::Parser;
use crate::error::PmtkError;
use crate::parse::number;
use crate::traits::{Message, Response};
use crate::types::DataField;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct EpoInfoDt {
    pub set: u16,
    pub fwn: u16,
    pub ftow: u32,
    pub lwn: u16,
    pub ltow: u32,
    pub fcwn: u16,
    pub fctow: u32,
    pub lcwn: u16,
    pub lctow: u32,
}

impl Message for EpoInfoDt {
    const PKT_TYPE: u16 = 707;
}

impl TryFrom<DataField> for EpoInfoDt {
    type Error = PmtkError;

    fn try_from(value: DataField) -> Result<Self, Self::Error> {
        let i = value.as_str();
        let (i, _) = char(',').parse(i)?;
        let (i, set) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, fwn) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, ftow) = number::<u32>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, lwn) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, ltow) = number::<u32>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, fcwn) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, fctow) = number::<u32>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (i, lcwn) = number::<u16>(i)?;
        let (i, _) = char(',').parse(i)?;
        let (_, lctow) = number::<u32>(i)?;

        Ok(EpoInfoDt {
            set,
            fwn,
            ftow,
            lwn,
            ltow,
            fcwn,
            fctow,
            lcwn,
            lctow
        })
    }
}

impl Response for EpoInfoDt {}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;

    #[test]
    fn try_from_data_field_ok() {
        let data_field = DataField::from_str(",28,1680,259200,1681,237600,1680,345600,1680,345600").unwrap();
        let epo_info = EpoInfoDt::try_from(data_field).unwrap();
        assert_eq!(epo_info.set, 28);
        assert_eq!(epo_info.fwn, 1680);
        assert_eq!(epo_info.ftow, 259200);
        assert_eq!(epo_info.lwn, 1681);
        assert_eq!(epo_info.ltow, 237600);
        assert_eq!(epo_info.fcwn, 1680);
        assert_eq!(epo_info.fctow, 345600);
        assert_eq!(epo_info.lcwn, 1680);
        assert_eq!(epo_info.lctow, 345600);
    }
}