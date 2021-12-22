use curl::easy::Easy;
use serde_json::{from_str, Value};

use crate::{get_effective, SimpleResult};


pub fn curl_to_string(url: &str) -> SimpleResult<String> {

    let mut curl = Easy::new();
    let mut buffer = Vec::new();

    curl.url(url)?;
    curl.follow_location(true)?;

    {
        let mut transfer = curl.transfer();

        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        })?;

        transfer.perform()?;
    }

    Ok(String::from_utf8(buffer)?)
}


pub fn get_new_ip() -> SimpleResult<String> {

    let json = curl_to_string("https://ipinfo.io")?;
    let json: Value = from_str(&json)?;

    if let Some(value) = json.get("ip") {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("No 'ip' key in JSON".into())
}


pub fn get_old_ip(gist_id: &str) -> SimpleResult<String> {

    let url = get_effective(&format!("https://gist.github.com/{}", gist_id))?;
    let url = format!("{}/raw/ip_data.json", url);

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