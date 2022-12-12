use serde::{Deserialize, Serialize};

/// Message to send to the front end
pub enum EmitMessages {
    GetSettings,
    GoToSettings,
    GoToTimer,
    PackageInfo,
    NextBreak,
    OnBreak,
    Paused,
    SendError,
    SessionsBeforeLong,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ShowTimer {
    interval: i64,
    strategy: String,
}
impl ShowTimer {
    pub const fn new(interval: i64, strategy: String) -> Self {
        Self { interval, strategy }
    }
}

/// These need to match the frontend types.InvokeMessage enum
impl EmitMessages {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::GetSettings => "get::settings",
            Self::GoToSettings => "goto::settings",
            Self::GoToTimer => "goto::timer",
            Self::NextBreak => "next-break",
            Self::OnBreak => "on-break",
            Self::SendError => "error",
            Self::SessionsBeforeLong => "sessions-before-long",
            Self::PackageInfo => "package-info",
            Self::Paused => "paused",
        }
    }
}
