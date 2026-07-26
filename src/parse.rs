use nom::Parser;
use core::str::FromStr;
use heapless::String;
use nom::bytes::complete::{take, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::preceded;
use crate::error::PmtkError;
use crate::types::PmtkPacket;

fn checksum(i: &str) -> IResult<&str, u8> {
    map_res(preceded(char('*'), take(2usize)), hex).parse(i)
}

fn hex(data: &str) -> Result<u8, &'static str> {
    u8::from_str_radix(data, 16).map_err(|_| "Failed to parse checksum as hex number")
}

fn num<I: FromStr>(data: &str) -> Result<I, &'static str> {
    data.parse::<I>().map_err(|_| "parse of number failed")
}

pub(crate) fn number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_res(digit1, num).parse(i)
}

pub(crate) fn number_in_range<T>(
    i: &str,
    lower_bound: T,
    upper_bound_inclusive: T,
) -> IResult<&str, T>
where
    T: PartialOrd + FromStr,
{
    map_res(number::<T>, |parsed_num| {
        if parsed_num < lower_bound || parsed_num > upper_bound_inclusive {
            return Err("Parsed number is outside of the expected range");
        }
        Ok(parsed_num)
    })
        .parse(i)
}

pub(crate) fn packet(i: &str) -> Result<PmtkPacket, PmtkError> {
    let (i, _) = talker_id(i)?;
    let (i, pkt_type) = packet_type(i)?;
    //let (i, _) = char(',').parse(i)?;
    let (i, data_field) = take_until("*").parse(i)?;
    let (_, checksum) = checksum(i)?;

    // TODO validate checksum?

    Ok(
        PmtkPacket {
            pkt_type,
            data_field: if !data_field.is_empty() {
                Some(String::from_str(data_field).map_err(|_| PmtkError::Parsing)?) // TODO is this the right error?
            } else {
                None
            },
            checksum,
        }
    )
}

fn packet_type(i: &str) -> IResult<&str, u16> {
    map_res(take(3usize), |packet_type: &str| {
        u16::from_str_radix(packet_type, 10).map_err(|_| PmtkError::Parsing)
    }).parse(i)
}

fn talker_id(i: &str) -> IResult<&str, &str> {
    map_res(preceded(char('$'), take(4usize)), |talker_id| {
        if talker_id != "PMTK" { return Err("Invalid talker id") } else { Ok(talker_id) }
    }).parse(i)
}