use std::error;
use std::time::SystemTime;
use chrono::{DateTime, Datelike};

use serde::{Serialize, Deserialize};
use curl::easy::Easy;


pub mod fetch;
pub mod update;


type SimpleResult<T> = Result<T, Box<dyn error::Error>>;


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

        let time: DateTime<chrono::Local> = self.since.into();

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



fn get_effective(url: &str) -> SimpleResult<String> {

    let mut curl = Easy::new();

    curl.url(url)?;
    curl.follow_location(true)?;
    curl.perform()?;

    if let Some(effective) = curl.effective_url()? {
        Ok(effective.to_owned())
    } else {
        Ok(url.to_owned())
    }

}