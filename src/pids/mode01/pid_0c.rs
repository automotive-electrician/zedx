// Engine Speed (RPM)

use crate::{
    consts::mode01::{MODE_01, names::PID_0C_NAME, pids::PID_0C, units::RPM},
    pids::mode01::{Pid, PidError},
};

pub struct EngineSpeed;
impl Pid for EngineSpeed {
    const MODE: u8 = MODE_01;
    const PID: u8 = PID_0C;
    const NAME: &'static str = PID_0C_NAME;
    const UNIT: Option<&'static str> = RPM;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<Self::Value, PidError> {
        let a = data.first().copied().ok_or(PidError::TooShort)?;
        let b = data.get(1).copied().ok_or(PidError::TooShort)?;
        Ok(f32::from(u16::from_be_bytes([a, b])) * 0.25)
    }
}
