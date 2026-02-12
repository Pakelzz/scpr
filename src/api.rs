use std::collections::HashMap;
use std::fs;
use std::io::Write;

use crate::model::{KotaResponse, Pray, ScheduleResponse};
use crate::error::MyError;
use crate::storage::Config;
use crate::validation::{ValidDate, time_validation};
use crate::{date};

pub async fn get_id(city: &str) -> Result<String, MyError> {
    let url = format!("https://api.myquran.com/v3/sholat/kabkota/cari/{}", city);
    let res = reqwest::get(url)
        .await?
        .json::<KotaResponse>()
        .await?;
    
    let mut map = HashMap::new();
    let mut count: i8 = 1;
    
    if res.data.len() > 1 {
        println!("No     Name");
    } 

    if res.data.len() == 1 {
        return Ok(res.data.first().unwrap().id.clone());
    }

    for i in res.data {
        println!("{}    {}", count, i.lokasi);
        map.insert(count, i.id);
        count += 1;
    }
    print!("Insert Number: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let input_int = input.trim().parse::<i8>()?;
    if let Some(id) = map.get(&input_int) {
        Ok(id.clone())
    } else {
        println!("Error get id");
        Err(MyError::OutOfRange)
    }
}

pub async fn get_schedule_today(id: &String) -> Result<Pray, MyError> {
    let url = format!("https://api.myquran.com/v3/sholat/jadwal/{}/today", id);
    let date = date::now();

    let res = reqwest::get(url)
        .await?
        .json::<ScheduleResponse>()
        .await?;

    let mut pray = Pray {
        kabko: res.data.kabko.clone(),
        ..Default::default()
    };

    if let Some(schedule) = res.data.jadwal.get(&date) {
        pray.fill_schedule(schedule, date);
        Ok(pray)
    } else {
        println!("{:?} {}", res.data.jadwal, date);
        Err(MyError::OutOfRange)
    }
}

pub async fn get_schedule_time(id: &str, time: &str) -> Result<Pray, MyError> {
    let date = time_validation(time, ValidDate::Full)?;
    let url = format!("https://api.myquran.com/v3/sholat/jadwal/{}/{}", id, date);
    let res = reqwest::get(url)
        .await?
        .json::<ScheduleResponse>()
        .await?;

    let mut pray = Pray {
        kabko: res.data.kabko.clone(),
        ..Default::default()
    };

    if let Some(schedule) = res.data.jadwal.get(&date.to_string()) {
        pray.fill_schedule(schedule, date.to_string());
        Ok(pray)
    } else {
        Err(MyError::OutOfRange)
    }   
    
}

// Fungsi dibawah dibuat untuk mengambil jadwal 1 bulan dan menulisnya di config
// Dan fungsi ini harus bekerja dengan cli.default dan akan diperbarui ketika tanggal 1
pub async fn get_schedule_month(id: &str, time: String) -> Result<ScheduleResponse, MyError> {
    let date = time_validation(&time, ValidDate::Month)?;
    let url = format!("https://api.myquran.com/v3/sholat/jadwal/{}/{}", id, date);
    let res = reqwest::get(url)
        .await?
        .json::<ScheduleResponse>()
        .await?;
    Ok(res)
}

pub fn get_id_local() -> Result<String, MyError> {
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("scpr/scpr.json");
        if !conf.exists() {
            return Err(MyError::ConfigNotFound);
        }
        let contents = fs::read_to_string(conf)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config.id)
    } else {
        Err(MyError::ConfigNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id_local() {
        match get_id_local() {
            Ok(x) => println!("{x}"),
            Err(e) => eprintln!("{e}")
        }
    }
}
