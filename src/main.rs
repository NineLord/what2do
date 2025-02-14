#![allow(unused)]
#![deny(unused_must_use)]

use std::{
	fs,
	path::{Path, PathBuf},
};
use git2::{Repository, Blame, ErrorCode};
use anyhow::Result;

fn handle_blame(absolute_path: &Path, relative_path: &Path, repo: &Repository, blame: Blame<'_>) {
	dbg!(&relative_path);
}

fn iter_recursively_all_files(absolute_path: PathBuf, mut relative_path: PathBuf, repo: &Repository) -> Result<()> {
	for entry in fs::read_dir(absolute_path)? {
		let entry = entry?;
		let file_type = entry.file_type()?;
		if file_type.is_file() {
			relative_path.push(Path::new(&entry.file_name()));
			match repo.blame_file(&relative_path, None) {
				Ok(blame) => handle_blame(&entry.path(), &relative_path, repo, blame),
				Err(error) => match error.code() {
					ErrorCode::NotFound => (), // New file that wasn't committed
					_ => unreachable!()
				},
			}
			relative_path.pop();
		} else if file_type.is_dir() {
			relative_path.push(Path::new(&entry.file_name()));
			match repo.blame_file(&relative_path, None) {
				Err(error) => {
					match error.code() {
						ErrorCode::InvalidSpec => // Can't blame directory but it is committed, so need to check it recursively.
							iter_recursively_all_files(entry.path(), relative_path.clone(), repo)?,
						ErrorCode::NotFound => (), // The directory wasn't committed, no need to check it recursively.
						_ => unreachable!()
					}
				},
				Ok(_) => unreachable!("Can't blame directory"),
			}
			relative_path.pop();
		}
	}
	Ok(())
}

fn main() -> Result<()> {
	let cwd = std::env::current_dir()?;
	let repo = Repository::open(&cwd)?;
	iter_recursively_all_files(cwd, Path::new("").to_path_buf(), &repo)?;
	// for x in repo.blame_file(Path::new("src/main.rs"), None)?.iter() {
	// 	println!("{:?}", x.final_signature().when());
	// }
	// println!("{}", repo.blame_file(Path::new("src/main.rs"), None)?.get_line(17).unwrap().lines_in_hunk());
	Ok(())
}
// todo
