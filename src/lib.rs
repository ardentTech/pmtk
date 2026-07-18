#![no_std]

use core::str::from_utf8;
use heapless::{format, String};
use nom::bytes::complete::{take, take_until};
use nom::character::complete::char;
use nom::{IResult, Parser};
use nom::combinator::map_res;
use nom::sequence::preceded;

const PACKET_LEN: usize = 255;
const PAYLOAD_LEN: usize = 246;

#[derive(Debug)]
pub enum PmtkError {
    Parsing
}
impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for PmtkError {
    fn from(_error: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::Parsing
    }
}

#[derive(Debug, PartialEq)]
struct PmtkPacket<'a> {
    data_field: &'a str, // TODO could be easier to use String here...
    pkt_type: u16
}

impl<'a> PmtkPacket<'a> {
    pub fn decode(raw: &'a str) -> Result<Self, PmtkError> {
        parse_packet(raw)
    }

    pub fn encode(&self) -> String<255> {
        let payload = format!(PAYLOAD_LEN; "PMTK{}{}", self.pkt_type, self.data_field).unwrap();
        let checksum = generate_checksum(payload.as_bytes());
        format!(PACKET_LEN; "${}*{:X?}\r\n", payload, checksum).unwrap()
    }
}

fn generate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &x| acc ^ x)
}

// parsing.rs

fn parse_packet(i: &str) -> Result<PmtkPacket<'_>, PmtkError> {
    let (i, _) = parse_talker_id(i)?;
    let (i, pkt_type) = parse_packet_type(i)?;
    //let (i, _) = char(',').parse(i)?;
    let (i, data_field) = take_until("*").parse(i)?;
    let (_, checksum) = parse_checksum(i)?;

    // TODO validate checksum

    Ok(
        PmtkPacket {
            pkt_type,
            data_field,
            //checksum,
        }
    )
}
fn parse_checksum(i: &str) -> IResult<&str, u8> {
    map_res(preceded(char('*'), take(2usize)), parse_hex).parse(i)
}

fn parse_hex(data: &str) -> Result<u8, &'static str> {
    u8::from_str_radix(data, 16).map_err(|_| "Failed to parse checksum as hex number")
}

fn parse_packet_type(i: &str) -> IResult<&str, u16> {
    map_res(take(3usize), |packet_type: &str| {
        u16::from_str_radix(packet_type, 10).map_err(|_| PmtkError::Parsing)
    }).parse(i)
}

fn parse_talker_id(i: &str) -> IResult<&str, &str> {
    map_res(preceded(char('$'), take(4usize)), |talker_id| {
        if talker_id != "PMTK" { return Err("Invalid talker id") } else { Ok(talker_id) }
    }).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_ok() {
        assert_eq!(PmtkPacket { data_field: ",1000", pkt_type: 220 }, PmtkPacket::decode("$PMTK220,1000*1F\r\n").unwrap());
    }

    #[test]
    fn encode_ok() {
        assert_eq!("$PMTK220,1000*1F\r\n", PmtkPacket { data_field: ",1000", pkt_type: 220 }.encode());
    }
}
