use crate::command::util::encode_data_field;
use crate::error::PmtkError;
use crate::packet::PmtkPacket;

// $PMTK251,38400*27\r\n
const PKT_TYPE: u16 = 251;

pub struct SetNmeaBaudrate {
    rate: u32,
}
impl SetNmeaBaudrate {
    pub fn new(rate: u32) -> Result<Self, PmtkError> {
        if ![0, 4800, 9600, 14400, 19200, 38400, 57600, 115200].contains(&rate) {
            return Err(PmtkError::InvalidBaudRate(rate));
        }
        Ok(Self { rate })
    }
}

impl Into<PmtkPacket> for SetNmeaBaudrate {
    fn into(self) -> PmtkPacket {
        let data_field = encode_data_field([self.rate]);
        PmtkPacket::new(data_field, PKT_TYPE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_pmtk_packet_ok() {
        let packet: PmtkPacket = SetNmeaBaudrate::new(38400).unwrap().into();
        assert_eq!("$PMTK251,38400*27\r\n", packet.encode());
    }

    #[test]
    fn new_err() {
        assert!(SetNmeaBaudrate::new(9601).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(SetNmeaBaudrate::new(9600).is_ok());
    }
}