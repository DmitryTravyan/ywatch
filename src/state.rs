use std::time::{Instant, SystemTime};
use chrono::{DateTime, Utc};
use serde_json::{Value, Map};
use std::collections::HashMap;

pub(crate) struct State {
    start_time: DateTime<Utc>,
    uptime: Instant,
}

impl State {
    pub(crate) fn init() -> Self {
        Self {
            start_time: Utc::now(),
            uptime: Instant::now(),
        }
    }

    pub(crate) fn get_json_as_str(&self) -> String {
        serde_json::to_string_pretty(&[
            (
                "start_time".to_string(),
                &self.start_time.to_rfc2822()
            ),
            (
                "uptime".to_string(),
                &self.uptime.elapsed().as_secs_f32().to_string()
            )
        ]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>()
        )
            .expect("nok")
    }
}