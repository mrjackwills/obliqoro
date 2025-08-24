use serde::{Deserialize, Serialize};

// TODO put these somehwere more useful
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CpuMeasure {
    pub current: f32,
    pub pause: Option<f32>,
    pub resume: Option<f32>,
}

/// This needs to match frontend types.FrontEndState
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct FrontEndState {
    pub auto_pause_threshold: u8,
    pub auto_pause_timespan_sec: u16,
    pub auto_pause: bool,
    pub auto_resume_threshold: u8,
    pub auto_resume_timespan_sec: u16,
    pub auto_resume: bool,
    pub fullscreen: bool,
    pub long_break_as_sec: u16,
    pub number_session_before_break: u8,
    pub paused: bool,
    pub session_as_sec: u16,
    pub short_break_as_sec: u16,
    pub start_on_boot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ShowTimer {
    interval: u16,
    strategy: String,
}

impl ShowTimer {
    pub const fn new(interval: u16, strategy: String) -> Self {
        Self { interval, strategy }
    }
}


