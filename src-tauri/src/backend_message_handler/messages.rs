use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::request_handlers::{CpuMeasure, FrontEndState};

/// Get information about self for the Footer component
/// BUILD_DATE is injected via the build.rs file
// TODO where is this used, why is it here?
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct PackageInfo {
    pub build_date: String,
    pub github_version: Option<String>,
    pub homepage: String,
    pub version: String,
}

impl Default for PackageInfo {
    fn default() -> Self {
        let (homepage, _) = env!("CARGO_PKG_REPOSITORY")
            .split_once(env!("CARGO_PKG_NAME"))
            .unwrap_or_default();
        Self {
            homepage: homepage.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            build_date: env!("BUILD_DATE").to_owned(),
            github_version: None,
        }
    }
}


#[derive(Debug, Clone)]
/// Front End Messages
pub enum MsgFE {
    Cpu(CpuMeasure),
    Error,
    GetSettings,
    GoToSettings,
    GoToTimer,
    NextBreak,
    OnBreak,
    PackageInfo(PackageInfo),
    Paused(bool),
    SessionsBeforeLong,
}

/// These need to match the frontend types.InvokeMessage const
impl MsgFE {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Cpu(_) => "cpu",
            Self::Error => "error",
            Self::GetSettings => "get::settings",
            Self::GoToSettings => "goto::settings",
            Self::GoToTimer => "goto::timer",
            Self::NextBreak => "next-break",
            Self::OnBreak => "on-break",
            Self::PackageInfo(_) => "package-info",
            Self::Paused(_) => "paused",
            Self::SessionsBeforeLong => "sessions-before-long",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
/// Break message
pub enum MsgB {
    End,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
/// Window Visiblity Messages
pub enum MsgWV {
    Close,
    Hide,
    Minimize,
    Show,
    Toggle,
}

#[derive(Debug, Clone)]
/// Heartbeat Message
pub enum MsgHB {
    Abort,
    OnHeartbeat(Option<f32>),
    Update(Arc<tokio::task::JoinHandle<()>>),
    UpdateTimer,
}

#[derive(Debug, Clone)]
/// InternalMessage
pub enum MsgI {
    Break(MsgB),
    HeartBeat(MsgHB),
    OpenLocation,
    Pause,
    ResetSettings,
    ResetTimer,
    SetSetting(FrontEndState),
    ToFrontEnd(MsgFE),
    UpdateMenuTimer,
    UpdatePause(bool),
    Window(MsgWV),
}
