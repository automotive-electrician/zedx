// Short term fuel trim (STFT)—Bank 2

use crate::{
    consts::mode01::{names::PID_08_NAME, pids::PID_08, units::PERCENTAGE},
    pids::mode01::{Pid, PidError},
};

pub struct ShortTermFuelTrimBank2;
impl Pid for ShortTermFuelTrimBank2 {
    const PID: u8 = PID_08;
    const NAME: &'static str = PID_08_NAME;
    const UNIT: Option<&'static str> = PERCENTAGE;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        Ok((f32::from(a) * 0.78125) - 100.00)
    }
}
