use std::fmt::Display;

use clap::ValueEnum;
use which::which;

use crate::errors::CrossLlvmError;

#[derive(Clone, ValueEnum)]
pub enum ContainerEngine {
    Docker,
    Podman,
}

impl Display for ContainerEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Docker => write!(f, "docker"),
            Self::Podman => write!(f, "podman"),
        }
    }
}

impl ContainerEngine {
    pub fn autodetect() -> Result<Self, CrossLlvmError> {
        if which("docker").is_ok() {
            Ok(Self::Docker)
        } else if which("podman").is_ok() {
            Ok(Self::Podman)
        } else {
            Err(CrossLlvmError::ContainerEngineNotFound)
        }
    }
}
