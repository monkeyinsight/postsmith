use dirs::config_dir;
use std::{fs, io::Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct History {
    date: String,
    action: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    history: Vec<History>
}

impl Session {
    pub fn new() -> Self {
        Self {
            history: Vec::new()
        }
    }

    pub fn push_history(&mut self, request: String) {
        let history = History {
            date: chrono::offset::Local::now().to_string(),
            action: request
        };

        let _ = &self.history.push(history);

        let _ = &self.save();
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(config_dir().unwrap().into_os_string().into_string().unwrap() + "/postsmith/session")?;

        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())?;
        return Ok(());
    }
}
