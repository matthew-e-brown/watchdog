use serde_json::{from_str, Value};
use crate::{get_effective, curl_to_string, StaticResult};


pub fn get_new_ip() -> StaticResult<String> {

    let json = curl_to_string("https://ipinfo.io")?;
    let json: Value = from_str(&json).or(Err("Did not receive valid JSON from server"))?;

    if let Some(value) = json.get("ip") {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("No 'ip' key in JSON response")
}


pub fn get_current_ip(gist_id: &str) -> StaticResult<String> {

    let url = get_effective(&format!("https://gist.github.com/{}", gist_id))?;
    let url = format!("{}/raw/ip_data.json", url);

    let json = curl_to_string(&url)?;
    let json: Value = from_str(&json).or(Err("Did not receive valid JSON from server"))?;

    let value = json.get("current")
        .and_then(|val| val.get("address"));

    if let Some(value) = value {
        if let Value::String(addr) = value {
            return Ok(addr.clone());
        }
    }

    Err("No 'current.address' key in JSON response")
}