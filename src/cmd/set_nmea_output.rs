use heapless::String;
use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::nmea_output::Frequency;
use crate::traits::{Cmd, Request, Packet};
use crate::packet::PmtkPacket;

// TODO impl default?
// TODO `new` method?

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaOutputCmd {
    pub gll: Frequency,
    pub rmc: Frequency,
    pub vtg: Frequency,
    pub gga: Frequency,
    pub gsa: Frequency,
    pub gsv: Frequency,
    pub mchn: Frequency,
}

impl Packet for SetNmeaOutputCmd {
    const PKT_TYPE: u16 = 314;
}

impl Request for SetNmeaOutputCmd {}

impl Cmd for SetNmeaOutputCmd {
    fn serialize(&self) -> Result<String<255>, PmtkError> {
        let data_field = encode_data_field([
            self.gll as u8,
            self.rmc as u8,
            self.vtg as u8,
            self.gga as u8,
            self.gsa as u8,
            self.gsv as u8,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            self.mchn as u8,
        ]);
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use crate::dt::nmea_output::Frequency::{Disabled, OnceEveryFivePositionFixes, OnceEveryOnePositionFix};
    use super::*;

    #[test]
    fn serialize_ok() {
        let cmd = SetNmeaOutputCmd {
            gll: OnceEveryOnePositionFix,
            rmc: OnceEveryOnePositionFix,
            vtg: OnceEveryOnePositionFix,
            gga: OnceEveryOnePositionFix,
            gsa: OnceEveryOnePositionFix,
            gsv: OnceEveryFivePositionFixes,
            mchn: Disabled
        };
        assert_eq!("$PMTK314,1,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0,0*2C\r\n", cmd.serialize().unwrap());
    }
}