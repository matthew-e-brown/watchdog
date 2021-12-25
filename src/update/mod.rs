use std::path::Path;

use crate::{BoxResult, IPResults, IPUpdate, DATA_FILENAME, MD_FILENAME};

use chrono::Utc;

mod data;
mod git;


pub fn clone_and_push(gist_id: &str, new_ip: &str, use_ssh: bool, use_utc: bool) -> BoxResult<()> {

    let dir = git::clone(gist_id, use_ssh).or(Err("Could not clone gist repository"))?;
    let path = dir.path();

    let update = IPUpdate {
        address: new_ip.to_owned(),
        since: Utc::now(),
    };

    let current_data = data::read_ip(path);

    let updated_data = if let Some(mut data) = current_data {
        data.previous.insert(0, data.current);
        data.previous.truncate(10);
        data.current = update;
        data
    } else {
        IPResults {
            current: update,
            previous: Vec::new()
        }
    };

    let ip_path = Path::new(path).join(DATA_FILENAME);
    let md_path = Path::new(path).join(MD_FILENAME);

    data::clear_folder(path)?;

    data::write_ip(&ip_path, &updated_data)?;
    data::write_md(&md_path, &updated_data, use_utc)?;

    let message = format!("IP Address update - {}", updated_data.current.format_time(use_utc));
    git::add_and_commit(path, message)?;
    git::push(path)?;

    Ok(())
}