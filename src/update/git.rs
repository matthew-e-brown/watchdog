use tempfile::TempDir;
use git2::{Repository, BranchType};

use crate::{SimpleResult, get_effective};


pub fn clone(gist_id: &str) -> SimpleResult<(TempDir, Repository)> {

    let url = format!("https://gist.github.com/{}", gist_id);
    let url = get_effective(&url)?;

    let dir = TempDir::new()?;

    let repo = Repository::clone(&url, dir.path())?;

    Ok((dir, repo))
}


// From:
// https://github.com/rust-lang/git2-rs/issues/561
pub fn commit<T, I>(repo: &Repository, paths: I) -> SimpleResult<()>
where
    T: git2::IntoCString,
    I: IntoIterator<Item = T>
{
    // Stage
    let mut index = repo.index()?;
    index.add_all(paths, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let head = repo.head()?;
    let author = repo.signature()?;

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

    Ok(())
}


pub fn push(repo: &Repository) -> SimpleResult<()> {

    Ok(())
}