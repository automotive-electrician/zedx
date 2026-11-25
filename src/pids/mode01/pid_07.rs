// Long term fuel trim (LTFT)—Bank 1

use crate::{
    consts::mode01::{names::PID_07_NAME, pids::PID_07, units::PERCENTAGE},
    pids::mode01::{Pid, PidError},
};

pub struct LongTermFuelTrimBank1;
impl Pid for LongTermFuelTrimBank1 {
    const PID: u8 = PID_07;
    const NAME: &'static str = PID_07_NAME;
    const UNIT: Option<&'static str> = PERCENTAGE;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        Ok((f32::from(a) * 0.78125) - 100.00)
    }
}
