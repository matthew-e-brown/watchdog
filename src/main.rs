use std::env;


fn main() -> Result<(), &'static str> {

    let gist = env::var("HOME_IP_GIST").or(Err("HOME_IP_GIST environment variable is not set."))?;

    let new_ip = ip_watcher::fetch_new_ip().or(Err("Could not get IP address from IP Info."))?;
    let old_ip = ip_watcher::check_old_ip(&gist).or(Err("Could not get old IP from GitHub Gist."))?;

    // Check if we need to update:
    if new_ip != old_ip {

        println!("IP address is out of date; need to update gist!");

    } else {

        println!("IP address is up to date; no need to update gist.");

    }

    Ok(())
}