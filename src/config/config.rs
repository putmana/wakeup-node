use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_token: String,
    pub alarm_file: String,
    pub audio_backend: String,
    pub audio_device: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            api_token: env::var("API_TOKEN").unwrap_or_else(|_| "secret".to_string()),
            alarm_file: env::var("ALARM_FILE").unwrap_or_else(|_| "/sounds/alarm.mp3".to_string()),
            audio_backend: env::var("AUDIO_BACKEND").unwrap_or_else(|_| "alsa".to_string()),
            audio_device: env::var("AUDIO_DEVICE").unwrap_or_else(|_| "alsa/hw:0,0".to_string()),
        }
    }
}
