use std::error::Error;
use std::time::SystemTime;
use chrono::{DateTime, Datelike, Local};

use serde::{Serialize, Deserialize};
use curl::easy::Easy;


pub mod fetch;
pub mod update;

pub type StaticResult<T> = Result<T, &'static str>;
pub type BoxResult<T> = Result<T, Box<dyn Error>>;

const DATA_FILENAME: &'static str = "data.json";
const MD_FILENAME: &'static str = "addresses.md";


#[derive(Debug, Serialize, Deserialize)]
pub struct IPUpdate {
    address: String,
    since: SystemTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IPResults {
    current: IPUpdate,
    previous: Vec<IPUpdate>,
}


impl IPUpdate {

    fn format_time(&self) -> String {

        let time: DateTime<Local> = self.since.into();

        let suffix = match time.day() {
            11 | 12 | 13 => "th",
            n if n % 10 == 1 => "st",
            n if n % 10 == 2 => "nd",
            n if n % 10 == 3 => "rd",
            _ => "th"
        };

        time.format(&format!("%B %-d{}, %Y", suffix)).to_string()
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