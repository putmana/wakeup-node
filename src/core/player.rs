use std::{path::Path, process::Stdio};

use anyhow::Result;
use tokio::process::{Child, Command};

use crate::core::error::CoreError;

pub struct AlarmPlayer {
    child: Option<Child>,
}

impl AlarmPlayer {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn is_active(&mut self) -> Result<bool, CoreError> {
        match self.child.as_mut() {
            Some(child) => match child.try_wait() {
                Ok(None) => Ok(true),
                Ok(Some(_)) => {
                    self.child = None;
                    Ok(false)
                }
                Err(error) => Err(CoreError::MpvInspect(format!("{error}"))),
            },
            None => Ok(false),
        }
    }

    pub async fn start(
        &mut self,
        alarm_file: &str,
        audio_backend: &str,
        audio_device: &str,
    ) -> Result<(), CoreError> {
        if !Path::new(alarm_file).exists() {
            return Err(CoreError::AlarmFileNotFound(alarm_file.to_string()));
        }

        if self.is_active()? {
            return Ok(());
        }

        let mut command: Command = Command::new("mpv");

        command
            .arg("--loop-file=inf")
            .arg("--no-video")
            .arg(format!("--ao={audio_backend}"))
            .arg(format!("--audio-device={audio_device}"))
            .arg(alarm_file)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child: Child = command
            .spawn()
            .map_err(|error| CoreError::MpvStart(format!("{error}")))?;

        self.child = Some(child);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), CoreError> {
        match self.child.as_mut() {
            Some(child) => match child.try_wait() {
                Ok(None) => {
                    child
                        .kill()
                        .await
                        .map_err(|error| CoreError::MpvStop(format!("{error}")))?;

                    let _ = child.wait().await;
                }
                Ok(Some(_)) => {}
                Err(error) => return Err(CoreError::MpvInspect(format!("{error}"))),
            },
            None => {}
        }

        self.child = None;
        Ok(())
    }
}
