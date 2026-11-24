// Short term fuel trim (STFT)—Bank 1

use crate::{
    consts::mode01::{names::PID_06_NAME, pids::PID_06, units::PERCENTAGE},
    pids::mode01::{Pid, PidError},
};

pub struct ShortTermFuelTrimBank1;
impl Pid for ShortTermFuelTrimBank1 {
    const PID: u8 = PID_06;
    const NAME: &'static str = PID_06_NAME;
    const UNIT: Option<&'static str> = PERCENTAGE;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        Ok((f32::from(1) * 0.78125) - 100.00)
    }
}
