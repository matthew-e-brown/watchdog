use std::io::Write;
use std::{fs, path::Path};
use crate::{SimpleResult, IPResults};

use super::DATA_FILENAME;


pub fn read_ip(repo_path: &Path) -> SimpleResult<Option<IPResults>> {

    let data = match fs::read_to_string(Path::new(repo_path).join(DATA_FILENAME)) {
        Ok(contents) => contents,
        Err(_) => return Ok(None), // we will need to create the file
    };

    let data: IPResults = serde_json::from_str(&data)?;
    Ok(Some(data))
}



pub fn write_ip(path: &Path, data: &IPResults) -> SimpleResult<()> {

    let mut file = fs::OpenOptions::new()
        .write(true)        // open in 'w' mode
        .create(true)       // create if does not exist
        .open(path)?;

    let data = serde_json::to_string_pretty(data)?;
    file.write(data.as_bytes())?;

    Ok(())
}



pub fn write_md(path: &Path, data: &IPResults) -> SimpleResult<()> {

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    // We will format the previous strings one time
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

    file.write(output.as_bytes())?;

    for update in previous.iter() {
        let output = format!("\n| {:<w1$} | {:>w2$} |", update.date, update.addr, w1=wid1, w2=wid2);
        file.write(output.as_bytes())?;
    }

    Ok(())
}