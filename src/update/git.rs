use std::path::Path;
use std::process::Command;

use crate::StaticResult;

use tempfile::TempDir;


pub fn clone(gist_id: &str, use_ssh: bool) -> StaticResult<TempDir> {

    let url = if use_ssh {
        format!("git@gist.github.com:{}.git", gist_id)
    } else {
        format!("https://gist.github.com/{}", gist_id)
    };

    let dir = TempDir::new().or(Err("Could not create temporary directory."))?;
    let path = dir.path().to_str().unwrap();

    // Clone into dir
    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("clone")
        .arg(&url)
        .arg(".")
        .status()
        .or(Err("Could not run `git`."))?;

    if status.success() {
        Ok(dir)
    } else {
        dir.close().unwrap();
        Err("Could not clone repository.")
    }
}


pub fn add_and_commit(repo_path: &Path, message: String) -> StaticResult<()> {

    let path = repo_path.to_str().unwrap();

    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("add")
        .arg("-A")
        .status()
        .or(Err("Could not run `git`."))?;

    if !status.success() {
        return Err("Could not stage changes");
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .status()
        .or(Err("Could not run `git`."))?;

    if !status.success() {
        return Err("Could not commit changes.");
    }

    Ok(())
}


pub fn push(repo_path: &Path) -> StaticResult<()> {

    let path = repo_path.to_str().unwrap();

    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("push")
        .arg("origin")
        .status()
        .or(Err("Could not run `git`."))?;

    if !status.success() {
        return Err("Could not push to repository.");
    }

    Ok(())
}