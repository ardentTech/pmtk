use crate::cmd::util::encode_data_field;
use crate::error::PmtkError;
use crate::dt::ack::AckDt;
use crate::dt::nmea_output::{Frequency, NmeaOutputDt};
use crate::traits::{Command, Message};
use crate::types::PmtkPacket;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct SetNmeaOutput {
    gll: Frequency,
    rmc: Frequency,
    vtg: Frequency,
    gga: Frequency,
    gsa: Frequency,
    gsv: Frequency,
    mchn: Frequency,
}

impl Message for SetNmeaOutput {
    const PKT_TYPE: u16 = 314;
}

impl Command for SetNmeaOutput {
    type R = AckDt;

    fn encode(&self) -> Result<PmtkPacket, PmtkError> {
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
        PmtkPacket::new_command(Self::PKT_TYPE, Some(data_field))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::dt::nmea_output::Frequency::{Disabled, OnceEveryFivePositionFixes, OnceEveryOnePositionFix};
    use crate::types::DataField;
    use super::*;

    #[test]
    fn encode_ok() {
        let cmd = SetNmeaOutput {
            gll: OnceEveryOnePositionFix,
            rmc: OnceEveryOnePositionFix,
            vtg: OnceEveryOnePositionFix,
            gga: OnceEveryOnePositionFix,
            gsa: OnceEveryOnePositionFix,
            gsv: OnceEveryFivePositionFixes,
            mchn: Disabled
        };
        let packet = PmtkPacket {
            checksum: 0x2c,
            data_field: Some(DataField::from_str(",1,1,1,1,1,5,0,0,0,0,0,0,0,0,0,0,0,0,0").unwrap()),
            pkt_type: SetNmeaOutput::PKT_TYPE,
        };
        assert_eq!(packet, cmd.encode().unwrap());
    }
}