// Calculated engine load

use crate::{
    consts::mode01::{MODE_01, names::PID_04_NAME, pids::PID_04, units::PERCENTAGE},
    pids::mode01::{Pid, PidError},
};

pub struct CalculatedEngineLoad;
impl Pid for CalculatedEngineLoad {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_04;
    const NAME: &'static str = PID_04_NAME;
    const UNIT: Option<&'static str> = PERCENTAGE;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<f32, PidError> {
        let a = data.first().copied().ok_or(PidError::TooShort)?;
        Ok(f32::from(a) * 0.39215686274)
    }
}
