use std::error;
use curl::easy::Easy;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, Value};


#[derive(Debug, Serialize, Deserialize)]
struct IPUpdate {
    address: String,
    since: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPResults {
    current: IPUpdate,
    previous: [IPUpdate; 10],
}


type StringResult = Result<String, Box<dyn error::Error>>;


pub fn curl_to_string(url: &str) -> StringResult {

    let mut curl = Easy::new();
    let mut buffer = Vec::new();

    curl.url(url).unwrap();

    {
        let mut transfer = curl.transfer();

        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform()?;
    }

    Ok(String::from_utf8(buffer)?)
}


pub fn fetch_new_ip() -> StringResult {

    let json = curl_to_string("https://ipinfo.io")?;
    let json: Value = from_str(&json)?;

    if let Some(value) = json.get("ip") {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("No 'ip' key in JSON".into())
}


pub fn check_old_ip(gist: &str) -> StringResult {

    let url = format!("https://gist.githubusercontent.com/{}/raw/ip_data.json", gist);

    let json = curl_to_string(&url)?;
    let json: Value = from_str(&json)?;

    let value = json.get("current")
        .and_then(|val| val.get("address"));

    if let Some(value) = value {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("No 'current.address' key in JSON".into())
}