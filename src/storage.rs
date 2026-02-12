use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{date, error::MyError, model::{Pray, Schedule, ScheduleResponse}};


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub id: String,
    pub kabko: String,
    pub jadwal: HashMap<String, Schedule>
}

impl Config {
    fn from(schedule: ScheduleResponse) -> Self {
        Self {
            id: schedule.data.id,
            kabko: schedule.data.kabko,
            jadwal: schedule.data.jadwal
        }
    }
}

pub fn write_config(schedule: ScheduleResponse) -> Result<(), MyError> {
    let config = Config::from(schedule);
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("scpr");

        let scpr_config = conf.join("scpr.json");
        if !conf.exists() {
            fs::create_dir_all(&conf)?;
        }

        let contents = serde_json::to_string_pretty(&config)?;
        fs::write(scpr_config, contents)?;
    } else {
        let mut config_path = PathBuf::new();
        if let Some(usr) = dirs::home_dir() 
            && let Some(home) = usr.to_str()
        {
            config_path.push(format!("{}/.config/scpr", home));
        }

        fs::create_dir_all(&config_path)?;
        let scpr_config = config_path.join("scpr.json");
        let contents = serde_json::to_string_pretty(&config)?;
        fs::write(scpr_config, contents)?
    }

    Ok(())
}

pub fn read_config() -> Result<(), MyError> {
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("scpr/scpr.json");
        if !conf.exists() {
            Err(MyError::ConfigNotFound)
        } else {
            let contents = fs::read_to_string(conf)?;
            let config = serde_json::from_str::<Config>(&contents)?;
            let mut pray = Pray {
                kabko: config.kabko.clone(),
                ..Default::default()
            };
            if let Some(sche) = config.jadwal.get(&date::now()) {
                pray.fill_schedule(sche, date::now());
                pray.print_schedule();
            } else {
                return Err(MyError::ConfigOutOfDate);
            }
            Ok(())
        }
    } else {
        Err(MyError::ConfigNotFound)
    }
}
