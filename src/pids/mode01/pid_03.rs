// Fuel system status

use crate::{
    consts::mode01::{MODE_01, names::PID_03_NAME, pids::PID_03},
    pids::mode01::{Pid, PidError},
};

pub struct FuelSystemStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelSystemState {
    Off,
    OpenLoopColdEngine,
    ClosedLoop,
    OpenLoopLoadOrDecelFuelCut,
    OpenLoopSystemFailure,
    ClosedLoopFeedbackFault,
    Unknown(u8),
}

impl From<u8> for FuelSystemState {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::Off,
            0x01 => Self::OpenLoopColdEngine,
            0x02 => Self::ClosedLoop,
            0x04 => Self::OpenLoopLoadOrDecelFuelCut,
            0x08 => Self::OpenLoopSystemFailure,
            0x10 => Self::ClosedLoopFeedbackFault,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuelSystemStatusValue {
    pub bank1: FuelSystemState,
    pub bank2: FuelSystemState,
}

impl Pid for FuelSystemStatus {
    const PID: u8 = PID_03;
    const NAME: &'static str = PID_03_NAME;
    const UNIT: Option<&'static str> = None;

    type Value = FuelSystemStatusValue;

    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        let b = *data.get(1).ok_or(PidError::TooShort)?;

        Ok(FuelSystemStatusValue {
            bank1: a.into(),
            bank2: b.into(),
        })
    }
}
