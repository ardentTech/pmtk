use crate::error::PmtkError;
use crate::packet::{PktType, PmtkPacket, SerializedPacket};
use crate::traits::{CmdQ, Packet};
use crate::util::encode_data_field;

const TIME_MIN: u32 = 1_000;
const TIME_MAX: u32 = 518_400_000;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OperationMode {
    #[default]
    Normal = 0x0,
    PeriodicBackup = 0x1,
    PeriodicStandby = 0x2,
    PerpetualBackup = 0x4,
    AlwaysLocateStandby = 0x8,
    AlwaysLocateBackup = 0x9,
}

type Ms = u32;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct PeriodicModeCmd {
    mode: OperationMode,
    run_time: Option<Ms>,
    sleep_time: Option<Ms>,
    second_run_time: Option<Ms>,
    second_sleep_time: Option<Ms>,
}
impl PeriodicModeCmd {
    pub fn new(
        mode: OperationMode,
        run_time: Option<Ms>,
        sleep_time: Option<Ms>,
        second_run_time: Option<Ms>,
        second_sleep_time: Option<Ms>
    ) -> Result<Self, PmtkError> {
        if let Some(run_time) = run_time {
            if !(TIME_MIN..=TIME_MAX).contains(&run_time) {
                return Err(PmtkError::OutOfRange(TIME_MIN, TIME_MAX, run_time))
            }
        }
        if let Some(sleep_time) = sleep_time {
            if !(TIME_MIN..=TIME_MAX).contains(&sleep_time) {
                return Err(PmtkError::OutOfRange(TIME_MIN, TIME_MAX, sleep_time))
            }
        }
        if let Some(second_run_time) = second_run_time {
            if !(TIME_MIN..=TIME_MAX).contains(&second_run_time) {
                return Err(PmtkError::OutOfRange(TIME_MIN, TIME_MAX, second_run_time))
            }
        }
        if let Some(second_sleep_time) = second_sleep_time {
            if !(TIME_MIN..=TIME_MAX).contains(&second_sleep_time) {
                return Err(PmtkError::OutOfRange(TIME_MIN, TIME_MAX, second_sleep_time))
            }
        }
        Ok(Self { mode, run_time, sleep_time, second_run_time, second_sleep_time })
    }
}

impl Packet for PeriodicModeCmd {
    const PKT_TYPE: PktType = [50, 50, 53]; // 225
}

impl CmdQ for PeriodicModeCmd {
    fn serialize(&self) -> Result<SerializedPacket, PmtkError> {
        let mut data_field = encode_data_field([self.mode as u32])?;
        if let Some(run_time) = self.run_time {
            data_field.push_str(&*encode_data_field([run_time])?)?;
        }
        if let Some(sleep_time) = self.sleep_time {
            data_field.push_str(&*encode_data_field([sleep_time])?)?;
        }
        if let Some(second_run_time) = self.second_run_time {
            data_field.push_str(&*encode_data_field([second_run_time])?)?;
        }
        if let Some(second_sleep_time) = self.second_sleep_time {
            data_field.push_str(&*encode_data_field([second_sleep_time])?)?;
        }

        PmtkPacket::new_request(Self::PKT_TYPE, Some(data_field))?.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::periodic_mode::OperationMode::{AlwaysLocateBackup, Normal, PeriodicBackup};

    #[test]
    fn serialize_always_locate_ok() {
        let cmd = PeriodicModeCmd {
            mode: AlwaysLocateBackup,
            run_time: None,
            sleep_time: None,
            second_run_time: None,
            second_sleep_time: None
        };
        assert_eq!("$PMTK225,9*22\r\n", cmd.serialize().unwrap());
    }

    #[test]
    fn serialize_periodic_ok() {
        let cmd = PeriodicModeCmd {
            mode: PeriodicBackup,
            run_time: Some(3000),
            sleep_time: Some(12000),
            second_run_time: Some(18000),
            second_sleep_time: Some(72000)
        };
        assert_eq!("$PMTK225,1,3000,12000,18000,72000*16\r\n", cmd.serialize().unwrap());
    }

    #[test]
    fn new_invalid_run_time_err() {
        assert!(PeriodicModeCmd::new(Normal, Some(TIME_MIN - 1), None, None, None).is_err());
    }

    #[test]
    fn new_invalid_sleep_time_err() {
        assert!(PeriodicModeCmd::new(Normal, None, Some(TIME_MAX + 1), None, None).is_err());
    }

    #[test]
    fn new_invalid_second_run_time_err() {
        assert!(PeriodicModeCmd::new(Normal, None, None, Some(TIME_MIN - 1), None).is_err());
    }

    #[test]
    fn new_invalid_second_sleep_time_err() {
        assert!(PeriodicModeCmd::new(Normal, None, None, None, Some(TIME_MAX + 1)).is_err());
    }

    #[test]
    fn new_ok() {
        assert!(PeriodicModeCmd::new(PeriodicBackup, None, None, None, None).is_ok());
    }
}