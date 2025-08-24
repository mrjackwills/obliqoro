use serde::{Deserialize, Serialize};

use crate::backend_message_handler::BuildInfo;

/// Message to send from Backend to Frontend
#[derive(Debug, Clone)]
pub enum ToFrontEnd {
    BuildInfo(BuildInfo),
    Cpu(CpuMeasure),
    Error,
	Fullscreen(bool),
    GetSettings,
    GoToSettings,
    GoToTimer,
    NextBreak,
    OnBreak,
    Paused(bool),
    SessionsBeforeLong,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
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

/// These need to match the frontend types.InvokeMessage const
impl ToFrontEnd {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::GetSettings => "get::settings",
            Self::GoToSettings => "goto::settings",
			Self::Fullscreen(_) => "fullscreen",
            Self::GoToTimer => "goto::timer",
            Self::NextBreak => "next-break",
            Self::OnBreak => "on-break",
            Self::Error => "error",
            Self::SessionsBeforeLong => "sessions-before-long",
            Self::BuildInfo(_) => "package-info",
            Self::Paused(_) => "paused",
            Self::Cpu(_) => "cpu",
        }
    }
}
