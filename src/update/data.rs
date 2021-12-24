use std::io::{self, Write};
use std::{fs, path::Path};

use crate::{DATA_FILENAME, IPResults};


/// Reads the IP Results from the cloned repository. Returns `None` if no valid JSON can be found.
pub fn read_ip(repo_path: &Path) -> Option<IPResults> {

    let file = Path::new(repo_path).join(DATA_FILENAME);

    let data = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => return None, // we will need to create the file
    };

    serde_json::from_str(&data).ok()
}


/// Deletes all the files in the repo to be replaced with other files
pub fn clear_folder(repo_path: &Path) -> io::Result<()> {

    let dir = fs::read_dir(repo_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if entry.file_name() == ".git" {
            continue;
        } else if entry.file_type()?.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}


pub fn write_ip(path: &Path, data: &IPResults) -> io::Result<()> {

    let mut file = fs::OpenOptions::new()
        .write(true)        // open in 'w' mode
        .create(true)       // create if does not exist
        .open(path)?;

    let data = serde_json::to_string_pretty(data)?;
    file.write(data.as_bytes())?;

    Ok(())
}



pub fn write_md(path: &Path, data: &IPResults) -> io::Result<()> {

    // We will format the 'previous' strings once and re-use
    struct Formatted {
        addr: String,
        date: String,
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

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

    let output = format!(
        "# IP Addresses\n\nSince {}, my public IP address has been **`{}`**.",
        current.format_time(), current.address
    );

    file.write(output.as_bytes())?;

    if previous.len() > 0 {
        let output = format!("\n\nThe previous ten IP addresses were:\n\n\
            | {: <w1$} | {: <w2$} |\n\
            | {:-<w1$} | {:->w2$} |",
            "From", "IP Address", ':', ':', w1=wid1, w2=wid2
        );

        file.write(output.as_bytes())?;

        for update in previous.iter() {
            let output = format!("\n| {:<w1$} | {:>w2$} |", update.date, update.addr, w1=wid1, w2=wid2);
            file.write(output.as_bytes())?;
        }
    }

    Ok(())
}