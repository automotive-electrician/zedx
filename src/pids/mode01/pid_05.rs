// Engine coolant temperature

use crate::{
    consts::mode01::{names::PID_05_NAME, pids::PID_05, units::CELSIUS},
    pids::mode01::{Pid, PidError},
};

pub struct EngineCoolantTemperature;

impl Pid for EngineCoolantTemperature {
    const PID: u8 = PID_05;
    const NAME: &'static str = PID_05_NAME;
    const UNIT: Option<&'static str> = CELSIUS;

    type Value = i16;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        Ok(i16::from(a) - 40)
    }
}
