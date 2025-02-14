#![allow(unused)]
#![deny(unused_must_use)]

use std::path::{Path, PathBuf};
use git2::{Repository, Worktree};
use anyhow::Result;

const CWD: &str = "/mnt/d/Users/Shaked/MyCleanSpace/Documents/000 VMware/Rust/what2do";

fn main() -> Result<()> {
	let mut repo = Repository::open(Path::new(CWD))?;
	for x in repo.blame_file(Path::new("src/main.rs"), None)?.iter() {
		println!("{}", x.lines_in_hunk());
	}
	Ok(())
}
// todo
