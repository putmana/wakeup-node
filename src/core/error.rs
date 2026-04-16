use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Alarm file not found: {0}")]
    AlarmFileNotFound(String),

    #[error("Failed to start mpv: {0}")]
    MpvStart(String),

    #[error("Failed to stop mpv: {0}")]
    MpvStop(String),

    #[error("Failed to inspect mpv process: {0}")]
    MpvInspect(String),
}
