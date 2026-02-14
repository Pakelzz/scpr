use time::{OffsetDateTime};
use time::macros::format_description;

use crate::error::*;


// Problematic
pub fn now() -> String {
    let now = OffsetDateTime::now_local().unwrap();
    let format = format_description!("[year]-[month]-[day]");
    now.format(&format).unwrap()
}

pub fn reformat(date: String) -> Result<String, MyError> {
    let mut parts = date.split('-');

    let year: i32 = parts
        .next()
        .ok_or(MyError::DateErr(DateError::InvalidFormat))?
        .parse()?;
    
    let month: u8 = parts
        .next()
        .ok_or(MyError::DateErr(DateError::InvalidFormat))?
        .parse()?;

    let day: u8 = parts
        .next()
        .ok_or(MyError::DateErr(DateError::InvalidFormat))?
        .parse()?;

    Ok(format!("{day}/{month}/{year}"))
}

pub fn clock_int() -> u32 {
    let date = OffsetDateTime::now_local().unwrap();
    date.format(&format_description!("[hour][minute]")).unwrap().parse().unwrap()
}
//
#[test]
fn test_date() {
    let date = OffsetDateTime::now_local().unwrap();
    let format = format_description!("[hour][minute]");
    let now = date.format(&format).unwrap();
    println!("{now}");
}
