use crate::{BoxResult, get_effective, DATA_FILENAME};

use curl::easy::Easy;
use serde_json::{from_str, Value};


fn curl_to_string(url: &str) -> BoxResult<String> {

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

        transfer.perform().or(Err(format!("`{}`: could not perform network request", url)))?;
    }

    let code = curl.response_code().unwrap();
    if code == 404 {
        return Err(format!("Have you run 'update' yet? 404 at {}", url).into());
    }

    String::from_utf8(buffer)
        .or(Err(format!("`{}`: could not convert response to String", url).into()))
}


pub fn get_new_ip() -> BoxResult<String> {

    let json = curl_to_string("https://ipinfo.io")?;
    let json: Value = from_str(&json).or(Err("Did not receive valid JSON from ipinfo.io's API"))?;

    if let Some(value) = json.get("ip") {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("Received unexpected response from ipinfo.io's API".into())
}


pub fn get_current_ip(gist_id: &str) -> BoxResult<String> {

    let url = get_effective(&format!("https://gist.github.com/{}", gist_id))?;
    let url = format!("{}/raw/{}", url, DATA_FILENAME);

    let json = curl_to_string(&url)?;
    let json: Value = from_str(&json).or(Err(format!("'{}' did not respond with JSON", url)))?;

    let value = json.get("current")
        .and_then(|val| val.get("address"));

    if let Some(value) = value {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err(format!("'{}' does not contain a '.current.address' field", url).into())
}