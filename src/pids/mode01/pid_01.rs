// Monitor status since DTCs cleared. (Includes malfunction indicator lamp (MIL), status and number of DTCs, components tests, DTC readiness checks)

use crate::{
    consts::mode01::{MODE_01, names::PID_01_NAME, pids::PID_01},
    pids::mode01::{Pid, PidError},
};

pub struct MonitorStatusSinceDTCsCleared;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorStatus {
    pub mil_on: bool,
    pub dtc_count: u8,
    pub is_diesel: bool,

    pub misfire_supported: bool,
    pub fuel_system_supported: bool,
    pub comprehensive_supported: bool,

    pub misfire_incomplete: bool,
    pub fuel_system_incomplete: bool,
    pub comprehensive_incomplete: bool,

    pub c: u8,
    pub d: u8,
}

impl Pid for MonitorStatusSinceDTCsCleared {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_01;
    const NAME: &'static str = PID_01_NAME;
    const UNIT: Option<&'static str> = None;

    type Value = MonitorStatus;
    fn decode(data: &[u8]) -> Result<MonitorStatus, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        let b = *data.get(1).ok_or(PidError::TooShort)?;
        let b = *data.get(2).ok_or(PidError::TooShort)?;
        let b = *data.get(3).ok_or(PidError::TooShort)?;

        Ok(MonitorStatus {
            mil_on: a & 0x80 != 0,
            dtc_count: a & 0x7F,
            is_diesel: b & 0x08 != 0,
            misfire_supported: b & 0x01 != 0,
            fuel_system_supported: b & 0x02 != 0,
            comprehensive_supported: b & 0x04 != 0,
            misfire_incomplete: b & 0x10 != 0,
            fuel_system_incomplete: b & 0x20 != 0,
            comprehensive_incomplete: b & 0x40 != 0,
            c,
            d,
        })
    }
}
