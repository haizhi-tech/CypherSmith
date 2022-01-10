use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Level {
    Info,
    Warn,
    Error,
    Bug,
}

impl Level {
    pub fn to_str(self) -> &'static str {
        match self {
            Level::Info => "Information",
            Level::Warn => "Warning",
            Level::Error => "Error",
            Level::Bug => "Internal Error",
        }
    }
}

/// An error during execution.
#[derive(Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub detail: Option<String>,
}

impl Diagnostic {
    pub fn new(level: Level, message: impl ToString, detail: impl Into<Option<String>>) -> Self {
        Diagnostic {
            level,
            message: message.to_string(),
            detail: detail.into(),
        }
    }

    // pub fn info(message: impl ToString, detail: impl Into<Option<String>>) -> Self {
    //     Diagnostic::new(Level::Info, message, detail)
    // }

    pub fn warn(message: impl ToString, detail: impl Into<Option<String>>) -> Self {
        Diagnostic::new(Level::Warn, message, detail)
    }

    // pub fn error(message: impl ToString, detail: impl Into<Option<String>>) -> Self {
    //     Diagnostic::new(Level::Error, message, detail)
    // }

    pub fn bug(message: impl ToString, detail: impl Into<Option<String>>) -> Self {
        Diagnostic::new(Level::Bug, message, detail)
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{}]: {}\n\t{}",
            self.level.to_str(),
            self.message,
            self.detail.as_deref().unwrap_or("")
        )?;
        Ok(())
    }
}
