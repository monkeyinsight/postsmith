use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

#[derive(Serialize, Deserialize, Debug)]
struct History {
    date: String,
    action: String,
    //  Danik is working on it
    //header: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    history: Vec<History>,
}

impl Session {
    pub fn new() -> Self {
        let mut session = Self {
            history: Vec::new(),
        };
        session.load().unwrap_or_default();
        session
    }

    pub fn push_history(&mut self, request: &str, /*header: String,*/ url: String) {
        let history = History {
            date: chrono::offset::Local::now().to_string(),
            action: request.to_string(),
            //header: header,
            url: url,
        };

        self.history.push(history);

        self.save().unwrap();
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = config_dir().unwrap().join("postsmith");
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?; // Create directory if it doesn't exist
        }
        let file_path = config_path.join("session");
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Overwrite existing file
            .open(file_path)?;

        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())?;
        Ok(())
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = config_dir().unwrap().join("postsmith/session");
        if file_path.exists() {
            let data = fs::read_to_string(file_path)?;
            let session: Session = serde_json::from_str(&data)?;
            self.history = session.history;
        }
        Ok(())
    }

    pub fn get_history(&self) -> String {
        self.history
            .iter()
            .map(|h| format!("{} - {} - {}\n", h.date, h.action, h.url))
            .collect()
    }
}
