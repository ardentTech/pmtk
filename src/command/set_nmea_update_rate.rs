use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::error::PmtkError::InvalidNmeaUpdateRate;
use crate::packet::PmtkPacket;

// $PMTK220,1000*1F\r\n
const PKT_TYPE: u16 = 220;
const MS_MIN: u32 = 100;
const MS_MAX: u32 = 10_000;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetNmeaUpdateRate {
    ms: u32
}

impl SetNmeaUpdateRate {
    pub fn new(ms: u32) -> Result<Self, PmtkError> {
        if !(MS_MIN..=MS_MAX).contains(&ms) {
            return Err(InvalidNmeaUpdateRate(ms))
        }
        Ok(Self { ms })
    }
}

impl Into<PmtkPacket> for SetNmeaUpdateRate {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.ms]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        let packet: PmtkPacket = SetNmeaUpdateRate { ms: 1000u32 }.into();
        assert_eq!("$PMTK220,1000*1F\r\n", packet.encode());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaUpdateRate::new(MS_MIN - 1).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaUpdateRate::new(MS_MIN).is_ok());
    }
}