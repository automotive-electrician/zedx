use crate::{
    consts::mode01::MODE_01,
    pids::mode01::{Pid, PidError},
};

pub struct SupportedPids<const P: u8, const START_PID: u8, const END_PID: u8>;

impl<const P: u8, const START_PID: u8, const END_PID: u8> Pid
    for SupportedPids<P, START_PID, END_PID>
{
    const MODE: u8 = MODE_01;
    const PID: u8 = P;
    const NAME: &'static str = "Supported PIDs";
    const UNIT: Option<&'static str> = None;

    type Value = u32;

    fn decode(data: &[u8]) -> Result<u32, PidError> {
        if data.len() < 4 {
            return Err(PidError::TooShort);
        }
        Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
    }
}

impl<const P: u8, const START_PID: u8, const END_PID: u8> SupportedPids<P, START_PID, END_PID> {
    /// بررسی پشتیبانی شدن یک PID در محدوده تعیین شده
    pub fn is_supported(mask: u32, target_pid: u8) -> bool {
        if target_pid < START_PID || target_pid > END_PID {
            return false;
        }
        let bit = END_PID - target_pid;
        (mask & (1 << bit)) != 0
    }
}

// تعریف نام‌های مستعار برای تمام بازه‌های استاندارد OBD-II
// pub type SupportedPid01to20 = SupportedPids<0x00, 0x01, 0x20>;
// pub type SupportedPid21to40 = SupportedPids<0x20, 0x21, 0x40>;
// pub type SupportedPid41to60 = SupportedPids<0x40, 0x41, 0x60>;
// pub type SupportedPid61to80 = SupportedPids<0x60, 0x61, 0x80>;
// pub type SupportedPid81toA0 = SupportedPids<0x80, 0x81, 0xA0>;
// pub type SupportedPidA1toC0 = SupportedPids<0xA0, 0xA1, 0xC0>;
