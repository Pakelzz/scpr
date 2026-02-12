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
