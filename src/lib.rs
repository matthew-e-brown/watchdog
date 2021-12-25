use chrono::{DateTime, Datelike, Local, Utc};
use std::error::Error;

use curl::easy::Easy;
use serde::{Deserialize, Serialize};

pub mod fetch;
pub mod update;

pub type StaticResult<T> = Result<T, &'static str>;
pub type BoxResult<T> = Result<T, Box<dyn Error>>;

const DATA_FILENAME: &'static str = "data.json";
const MD_FILENAME: &'static str = "addresses.md";


#[derive(Debug, Serialize, Deserialize)]
pub struct IPUpdate {
    address: String,
    since: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IPResults {
    current: IPUpdate,
    previous: Vec<IPUpdate>,
}


impl IPUpdate {

    fn get_suffix(day: u32) -> &'static str {
        match day {
            11 | 12 | 13 => "th",
            n if n % 10 == 1 => "st",
            n if n % 10 == 2 => "nd",
            n if n % 10 == 3 => "rd",
            _ => "th"
        }
    }

    pub fn format_time(&self, utc: bool) -> String {
        if utc {

            let suffix = Self::get_suffix(self.since.day());
            let fmt = format!("%B %-d{} %Y at %H:%M", suffix);
            self.since.format(&fmt).to_string()

        } else {

            let time: DateTime<Local> = self.since.into();
            let suffix = Self::get_suffix(time.day());
            let fmt = format!("%B %-d{} %Y at %-I:%M %P", suffix);
            time.format(&fmt).to_string()

        }
    }

}


fn get_effective(url: &str) -> StaticResult<String> {

    let mut curl = Easy::new();

    curl.url(url).unwrap();
    curl.follow_location(true).unwrap();
    curl.perform().or(Err("Could not perform network request"))?;

    if let Some(effective) = curl.effective_url().unwrap() {
        Ok(effective.to_owned())
    } else {
        Ok(url.to_owned())
    }

}