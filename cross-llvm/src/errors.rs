use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrossLlvmError {
    #[error("no supported container engine (docker, podman) was found")]
    ContainerEngineNotFound,
}
