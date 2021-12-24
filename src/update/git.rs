use tempfile::TempDir;
use git2::Repository;

use crate::{StaticResult, get_effective};


pub fn clone(gist_id: &str) -> StaticResult<(TempDir, Repository)> {

    let url = format!("https://gist.github.com/{}", gist_id);
    let url = get_effective(&url)?;

    let dir = TempDir::new().unwrap();
    let repo = Repository::clone(&url, dir.path()).or(Err("Could not clone gist"))?;

    Ok((dir, repo))
}


pub fn commit<T, I>(repo: &Repository, paths: I)
where
    T: git2::IntoCString,
    I: IntoIterator<Item = T>
{
    // Stage
    let mut index = repo.index().unwrap();
    index.add_all(paths, git2::IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();

    let head = repo.head().unwrap();
    let author = repo.signature().unwrap();

    // Commit
    // let oid = index.write_tree()?;
    // let sig = repo.signature()?;
    // let parent = repo.head()?.peel_to_commit()?;
    // let tree = repo.find_tree(oid)?;

    // repo.commit(
    //     Some("HEAD"),
    //     &sig,
    //     &sig,
    //     "message",
    //     &tree,
    //     &[&parent]
    // )?;
}


pub fn push(repo: &Repository) -> StaticResult<()> {

    Ok(())
}