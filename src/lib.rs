use std::time::SystemTime;
use chrono::{DateTime, Datelike};

use serde::{Serialize, Deserialize};
use curl::easy::Easy;


pub mod fetch;
pub mod update;

type StaticResult<T> = Result<T, &'static str>;


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



fn curl_to_string(url: &str) -> StaticResult<String> {

    let mut curl = Easy::new();
    let mut buffer = Vec::new();

    curl.url(url).unwrap();
    curl.follow_location(true).unwrap();

    {
        let mut transfer = curl.transfer();

        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().or(Err("Could not perform network request"))?;
    }

    String::from_utf8(buffer).or(Err("Could not convert network response to String"))
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