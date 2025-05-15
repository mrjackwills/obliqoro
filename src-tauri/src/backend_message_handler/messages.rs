use serde::{Deserialize, Serialize};

use crate::request_handlers::{FrontEndState, MsgToFrontend};

/// Get information about self for the Footer component
/// BUILD_DATE is injected via the build.rs file
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct BuildInfo {
    pub homepage: String,
    pub version: String,
    pub build_date: String,
    pub github_version: Option<String>,
}
impl Default for BuildInfo {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum BreakMessage {
    End,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum WindowVisibility {
    Close,
    Hide,
    Minimize,
    Show,
    Toggle,
}

#[derive(Debug, Clone)]
pub enum InternalMessage {
    Break(BreakMessage),
    Pause,
    ResetSettings,
    ResetTimer,
    SetSetting(FrontEndState),
    ToFrontEnd(MsgToFrontend),
    UpdateMenuTimer,
    Window(WindowVisibility),
}
