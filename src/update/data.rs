use std::io::Write;
use std::{fs, path::Path};
use crate::IPResults;

use super::DATA_FILENAME;


/// Reads the IP Results from the cloned repository
pub fn read_ip(repo_path: &Path) -> Option<IPResults> {

    let file = Path::new(repo_path).join(DATA_FILENAME);

    let data = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => return None, // we will need to create the file
    };

    let data = serde_json::from_str::<IPResults>(&data);
    if let Err(_) = data {
        None
    } else {
        Some(data.unwrap())
    }
}


pub fn clear_folder(path: &Path) {

    let dir = fs::read_dir(path).unwrap();

    for file in dir {
        let file = file.unwrap();
    }
}


pub fn write_ip(path: &Path, data: &IPResults) {

    let mut file = fs::OpenOptions::new()
        .write(true)        // open in 'w' mode
        .create(true)       // create if does not exist
        .open(path).unwrap();

    let data = serde_json::to_string_pretty(data).unwrap();
    file.write(data.as_bytes()).unwrap();
}



pub fn write_md(path: &Path, data: &IPResults) {

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path).unwrap();

    // We will format the 'previous' strings once and re-use
    struct Formatted {
        addr: String,
        date: String,
    }

    let current = &data.current;
    let previous: Vec<_> = data.previous.iter().map(|update| {
        Formatted {
            addr: format!("`{}`", update.address),
            date: update.format_time(),
        }
    }).collect();

    // Find the widths of the markdown columns
    let (wid1, wid2) = previous.iter().fold((0, 0), |mut a, c| {

        let w1 = c.date.len();
        let w2 = c.addr.len();

        if w1 > a.0 { a.0 = w1; }
        if w2 > a.1 { a.1 = w2; }

        a
    });

    let output = format!(r##"# IP Addresses

Since {}, the IP address has been **`{}`**.

The previous ten IP addresses were:

| {:<w1$} | {:<w2$} |
| {:<-w1$} | {:>-w2$} |"##,
        current.format_time(), current.address, "From", "IP Address", ':', ':', w1=wid1, w2=wid2
    );

    file.write(output.as_bytes()).unwrap();

    for update in previous.iter() {
        let output = format!("\n| {:<w1$} | {:>w2$} |", update.date, update.addr, w1=wid1, w2=wid2);
        file.write(output.as_bytes()).unwrap();
    }

}