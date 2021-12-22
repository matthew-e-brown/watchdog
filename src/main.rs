use std::env;

use ip_watcher::{fetch, update};


fn main() -> Result<(), &'static str> {

    let gist = env::var("GIST_ID").or(Err("GIST_ID environment variable is not set."))?;

    let new_ip = fetch::get_new_ip().or(Err("Could not get IP address from IP Info."))?;
    let old_ip = fetch::get_old_ip(&gist).or(Err("Could not get old IP from GitHub Gist."))?;

    println!("New: {}\nOld: {}", new_ip, old_ip);

    // Check if we need to update:
    if new_ip != old_ip {

        println!("IP address is out of date; need to update gist!");

    } else {

        println!("IP address is up to date; no need to update gist.");

    }

    Ok(())
}