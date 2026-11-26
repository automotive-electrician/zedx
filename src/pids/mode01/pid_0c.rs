// Engine Speed (RPM)

use crate::{
    consts::mode01::{MODE_01, names::PID_0C_NAME, pids::PID_0C, units::RPM},
    pids::mode01::{Pid, PidError},
};

pub struct EngineSpeed;
impl Pid for EngineSpeed {
    const PID: u8 = PID_0C;
    const NAME: &'static str = PID_0C_NAME;
    const UNIT: Option<&'static str> = RPM;

    type Value = f32;
    fn decode(data: &[u8]) -> Result<f32, PidError> {
        let a = data.first().copied().ok_or(PidError::TooShort)?;
        let b = data.get(1).copied().ok_or(PidError::TooShort)?;
        Ok(f32::from(u16::from_be_bytes([a, b])) * 0.25)
    }
}

// Tests

// strip pci tests
#[cfg(test)]
mod strip_pci_test {
    use crate::pids::mode01::{Pid, PidError, pid_0c::EngineSpeed};

    fn strip(raw: &[u8]) -> Result<&[u8], PidError> {
        EngineSpeed::strip_pci(raw)
    }

    #[test]
    fn standard_can_frame() {
        let raw = [0x04, 0x41, 0x0C, 0xA8, 0x85, 0x00, 0x00, 0x00];
        // assert_eq!(strip(&raw), Ok(&[0x41, 0x0C, 0xA8, 0x85][..]));
        println!("{:#?}", strip(&raw))
    }

    #[test]
    fn not_single_frame() {
        let raw = [0xFF, 0x41, 0x0C, 0xA8, 0x85, 0x00, 0x00, 0x00];
        println!("{:#?}", strip(&raw))
    }

    #[test]
    fn bad_length() {
        let raw = [0x04, 0x41, 0x0C, 0x77];
        println!("{:#?}", strip(&raw))
    }
}

// validate response
#[cfg(test)]
mod validate_response_test {
    use super::*;

    #[test]
    fn standard_response() {
        let payload = [0x41, 0x0C, 0xA8, 0x20];
        println!("{:#?}", EngineSpeed::validate_response(&payload))
    }

    #[test]
    fn short_response() {
        let payload = [0x41, 0x0C];
        println!("{:#?}", EngineSpeed::validate_response(&payload))
    }

    #[test]
    fn nrc_response() {
        let payload = [0x7F, 0x0C, 0xA8, 0x20];
        println!("{:#?}", EngineSpeed::validate_response(&payload))
    }

    #[test]
    fn mode_response() {
        let payload = [0x85, 0x0C, 0xA8, 0x20];
        println!("{:#?}", EngineSpeed::validate_response(&payload))
    }

    #[test]
    fn pid_response() {
        let payload = [0x41, 0x0D, 0xA8, 0x20];
        println!("{:#?}", EngineSpeed::validate_response(&payload))
    }
}

// decode tests
#[cfg(test)]
mod engine_rpm_test {
    use crate::pids::mode01::{Pid, pid_0c::EngineSpeed};

    #[test]
    fn decode() {
        let data = [0x2A, 0x01];
        assert_eq!(EngineSpeed::decode(&data), Ok(2688.25));
        println!("{:#?}", EngineSpeed::decode(&data))
    }

    #[test]
    fn decode_max() {
        let data = [0xFF, 0xFF];
        assert_eq!(EngineSpeed::decode(&data), Ok(16383.75));
        println!("{:#?}", EngineSpeed::decode(&data))
    }

    #[test]
    fn decode_min() {
        let data = [0x00, 0x00];
        assert_eq!(EngineSpeed::decode(&data), Ok(0.00));
        println!("{:#?}", EngineSpeed::decode(&data))
    }
}

// process
#[cfg(test)]
mod process_test {
    use super::*;

    #[test]
    fn standard_process() {
        let raw = [0x04, 0x41, 0x0C, 0x68, 0x85, 0x00, 0x00, 0x00];
        println!("{:#?}", EngineSpeed::process(&raw))
    }
}
