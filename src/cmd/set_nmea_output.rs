use crate::dt::nmea_output::Frequency;
use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default)]
pub struct SetNmeaOutputCmd {
    gll: Frequency,
    rmc: Frequency,
    vtg: Frequency,
    gga: Frequency,
    gsa: Frequency,
    gsv: Frequency,
    mchn: Frequency,
}

impl SetNmeaOutputCmd {
    pub fn new(
        gll: Frequency,
        rmc: Frequency,
        vtg: Frequency,
        gga: Frequency,
        gsa: Frequency,
        gsv: Frequency,
        mchn: Frequency,
    ) -> Self {
        Self { gll, rmc, vtg, gga, gsa, gsv, mchn }
    }
}

impl Packet for SetNmeaOutputCmd {
    const PKT_TYPE: PktType = [51, 49, 52]; // 314
}

impl CmdQ for SetNmeaOutputCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let data_field = encode_data_field([
            self.gll as u8,
            self.rmc as u8,
            self.vtg as u8,
            self.gga as u8,
            self.gsa as u8,
            self.gsv as u8,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            self.mchn as u8,
        ])?;
        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dt::nmea_output::Frequency::{Disabled, OnceEveryFivePositionFixes, OnceEveryOnePositionFix};

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