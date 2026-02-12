use std::fmt::Display;

use time::{Date, Month, macros::format_description,};

use crate::error::MyError;
use crate::error::DateError::{InvalidFormat, InvalidMonth};

pub enum ValidDate {
    Full,
    Month,
}

pub enum ParsedDate {
    Full(Date),
    Month { year: i32, month: Month}
}

impl Display for ParsedDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedDate::Full(date) => {
                write!(f, "{date}")
        },
            ParsedDate::Month { year, month } => {
                write!(f, "{year}-{:02}", *month as u8)
            }
        }
    }
}

pub fn time_validation(time: &str, kind: ValidDate) -> Result<ParsedDate, MyError> {
    match kind {
        ValidDate::Full => {
            let desc = format_description!("[year]-[month]-[day]");
            let date = Date::parse(time, desc)?;
            Ok(ParsedDate::Full(date))
        },
        ValidDate::Month => {
            let mut parts = time.split('-');

            let year: i32 = parts
                .next()
                .ok_or(MyError::DateErr(InvalidFormat))?
                .parse()?;

            let month: u8 = parts
                .next()
                .ok_or(MyError::DateErr(InvalidFormat))?
                .parse()?;

            if !(1..=12).contains(&month) {
                return Err(MyError::DateErr(InvalidMonth));
            }

            Ok(ParsedDate::Month {
                year,
                month: Month::try_from(month)?,
            })
        }
    }
}
