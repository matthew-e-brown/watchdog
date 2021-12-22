use std::path::Path;
use std::time::SystemTime;

use crate::{SimpleResult, IPResults, IPUpdate};

const DATA_FILENAME: &'static str = "data.json";
const MD_FILENAME: &'static str = "addresses.md";


mod git;
mod data;


pub fn push_new_ip(gist_id: &str, new_ip: String) -> SimpleResult<()> {

    let update = IPUpdate { address: new_ip, since: SystemTime::now() };
    let (dir, repo) = git::clone(gist_id)?;

    // Will error if there is no workdir. Since we are the ones cloning it, we know it won't be bare and there will
    // always be a workdir; we may `unwrap`.
    let path = repo.workdir().unwrap();

    let new_data = if let Some(mut data) = data::read_ip(path)? {
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

    // Serialize and write to file
    data::write_ip(&ip_path, &new_data)?;
    data::write_md(&md_path, &new_data)?;

    // Commit changes
    git::commit(&repo, &[ ip_path, md_path ])?;

    // Push changes


    std::mem::drop(repo); // Need to drop repo before temp dir can be freed
    dir.close()?;
    Ok(())
}





#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn clone_and_read() {

        let example_id = "2be930b9e8f00d34619f748efb54def3";

        let (dir, repo) = git::clone(&example_id).unwrap();

        println!("repo path: {:?}", repo.workdir());

        std::mem::drop(repo); // Drop repo before attempting to delete directory
        dir.close().unwrap();
    }

}