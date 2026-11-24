use crate::{
    consts::mode01::MODE_01,
    pids::mode01::{pid_01::MonitorStatus, pid_03::FuelSystemStatusValue},
};

pub mod pid_01;
pub mod pid_03;
pub mod pid_04;
pub mod pid_05;
pub mod pid_06;
pub mod pid_0c;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PidValue {
    U8(u8),
    U16(u16),
    F32(f32),
    I16(i16),
    U32(u32),
    MonitorStatus(MonitorStatus),
    FuelSystemStatus(FuelSystemStatusValue),
}

impl From<u8> for PidValue {
    fn from(v: u8) -> Self {
        Self::U8(v)
    }
}

impl From<u16> for PidValue {
    fn from(v: u16) -> Self {
        Self::U16(v)
    }
}

impl From<f32> for PidValue {
    fn from(v: f32) -> Self {
        Self::F32(v)
    }
}

impl From<i16> for PidValue {
    fn from(v: i16) -> Self {
        Self::I16(v)
    }
}

impl From<u32> for PidValue {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}

impl From<MonitorStatus> for PidValue {
    fn from(v: MonitorStatus) -> Self {
        Self::MonitorStatus(v)
    }
}

impl From<FuelSystemStatusValue> for PidValue {
    fn from(v: FuelSystemStatusValue) -> Self {
        Self::FuelSystemStatus(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PidError {
    TooShort,
    NotSingleFrame,
    BadLength,
    WrongMode,
    WrongPid,
    NegativeResponse(u8),
}

pub trait Pid {
    const MODE: u8 = MODE_01;
    const PID: u8;
    const NAME: &'static str;
    const UNIT: Option<&'static str>;

    type Value: Into<PidValue>;

    fn strip_pci<'a>(raw_frame: &'a [u8]) -> Result<&'a [u8], PidError> {
        if raw_frame.is_empty() {
            return Err(PidError::TooShort);
        }

        let pci = raw_frame[0];
        let frame_type = pci >> 4;
        let len = (pci & 0x0F) as usize;

        if frame_type != 0 {
            return Err(PidError::NotSingleFrame);
        }

        if raw_frame.len() < 1 + len {
            return Err(PidError::BadLength);
        }

        Ok(&raw_frame[1..1 + len])
    }

    fn validate_response<'a>(payload: &'a [u8]) -> Result<&'a [u8], PidError> {
        if payload.len() < 2 {
            return Err(PidError::TooShort);
        }

        if payload[0] == 0x7F {
            let nrc = payload.get(2).copied().unwrap_or(0);
            return Err(PidError::NegativeResponse(nrc));
        }

        if payload[0] != Self::MODE + 0x40 {
            return Err(PidError::WrongMode);
        }

        if payload[1] != Self::PID {
            return Err(PidError::WrongPid);
        }

        Ok(&payload[2..])
    }

    fn decode(data: &[u8]) -> Result<Self::Value, PidError>;

    fn process(raw_frame: &[u8]) -> Result<PidValue, PidError> {
        let payload = Self::strip_pci(raw_frame)?;
        let data = Self::validate_response(payload)?;
        let value = Self::decode(data)?;
        Ok(value.into())
    }
}
