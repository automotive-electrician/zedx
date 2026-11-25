// Fuel pressure

use crate::{
    consts::mode01::{names::PID_0A_NAME, pids::PID_0A, units::KILOPASCAL},
    pids::mode01::{Pid, PidError},
};

pub struct FuelPressure;
impl Pid for FuelPressure {
    const PID: u8 = PID_0A;
    const NAME: &'static str = PID_0A_NAME;
    const UNIT: Option<&'static str> = KILOPASCAL;

    type Value = u16;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = *data.first().ok_or(PidError::TooShort)?;
        Ok(u16::from(a) * 3)
    }
}
