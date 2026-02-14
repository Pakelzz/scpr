use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::date::reformat;

// Untuk ngambil jadwal 
#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleResponse {
    pub data: Data
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: String,
    pub kabko: String,
    pub jadwal: HashMap<String, Schedule>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String
}

// Untuk ngambil kabkota
#[derive(Debug, Deserialize, Serialize)]
pub struct KotaResponse {
    pub status: bool,
    pub data: Vec<Kota>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kota {
    pub id: String,
    pub lokasi: String
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Pray {
    pub kabko: String,
    pub date: String,
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

impl Pray {
    pub fn print_schedule(&self) {
        println!("Prayer schedule for {}, {}", self.kabko, self.date);
        println!("Subuh\t{}", self.subuh);
        println!("Dzuhur\t{}", self.dzuhur);
        println!("Ashar\t{}", self.ashar);
        println!("Magrib\t{}", self.maghrib);
        println!("Isya\t{}", self.isya);
    }

    pub fn fill_schedule(&mut self, schedule: &Schedule, date: String) {
        self.date = reformat(date).unwrap();
        self.subuh = schedule.subuh.clone();
        self.dzuhur= schedule.dzuhur.clone();
        self.ashar = schedule.ashar.clone();
        self.maghrib = schedule.maghrib.clone();
        self.isya = schedule.isya.clone();
    }

}
