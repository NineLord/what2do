#![allow(unused)]
#![deny(unused_must_use)]

use std::{
	fs,
	ffi::OsString,
	env,
	str::from_utf8,
	time::{Duration, SystemTime, UNIX_EPOCH},
	path::{Path, PathBuf},
	cell::LazyCell,
};
use git2::{Blame, BlameHunk, BlameOptions, ErrorCode, Repository, Time};
use anyhow::Result;
use regex::{Regex, RegexBuilder};

struct LineData {
	when: SystemTime,
	line: String,
	line_number: usize,
	file_path: OsString,
}

impl LineData {
	fn new_everything(when: SystemTime, line: &str, line_number: usize, file_path: &Path) -> Self {
		Self {
			when,
			line: String::from(line),
			line_number,
			file_path: file_path.to_path_buf().into_os_string(),
		}
	}
}

struct Lines {
	lines: Vec<LineData>,
	now: SystemTime,
}

impl Lines {
	pub fn new() -> Self {
		Self {
			lines: vec![],
			now: SystemTime::now(),
		}
	}

	pub fn push_committed(&mut self, when: &BlameHunk<'_>, line: &str, line_number: usize, file_path: &Path) {
		self.lines.push(LineData::new_everything(
			UNIX_EPOCH + Duration::from_secs(when.final_signature().when().seconds() as u64),
			line, line_number, file_path
		));
	}

	pub fn push_uncommitted(&mut self, line: &str, line_number: usize, file_path: &Path) {
		self.lines.push(LineData::new_everything(self.now, line, line_number, file_path));
	}
}

struct Foo {
	starting_path: PathBuf,
	repository: Repository,
	todo_regex: Regex,
}

impl Foo {
	pub fn new(path: PathBuf) -> Result<Self> {
		Ok(Self {
			starting_path: path.clone(),
			repository: Repository::open(path)?,
			todo_regex: RegexBuilder::new("TODO")
				.case_insensitive(true)
				.build()
				.expect("Hard coded regex, shouldn't fail"),
		})
	}
}

impl Foo {
	pub fn start(&self) -> Result<Lines> {
		let mut lines = Lines::new();
		self.iter_recursively_all_files(&self.starting_path, PathBuf::new(), &mut lines)?;
		Ok(lines)
	}

	fn iter_recursively_all_files(&self, absolute_path: &PathBuf, mut relative_path: PathBuf, lines: &mut Lines) -> Result<()> {
		for entry in fs::read_dir(absolute_path)? {
			let entry = entry?;
			let file_type: fs::FileType = entry.file_type()?;
			if file_type.is_file() {
				relative_path.push(Path::new(&entry.file_name()));
				match self.repository.blame_file(&relative_path, Some(BlameOptions::new()
					.track_copies_any_commit_copies(true)
					.track_copies_same_commit_copies(true)
					.track_copies_same_commit_moves(true)
					.track_copies_same_file(true) // TODO: maybe can do it without options? or use repo.status_should_ignore?
				)) {
					Ok(blame) => self.handle_file(&entry.path(), &relative_path, blame, lines)?,
					Err(error) => match error.code() {
						ErrorCode::NotFound => (), // New file that wasn't committed
						_ => unreachable!()
					},
				}
				relative_path.pop();
			} else if file_type.is_dir() {
				relative_path.push(Path::new(&entry.file_name()));
				match self.repository.blame_file(&relative_path, None) {
					Err(error) => {
						match error.code() {
							ErrorCode::InvalidSpec => // Can't blame directory but it is committed, so need to check it recursively.
								self.iter_recursively_all_files(&entry.path(), relative_path.clone(), lines)?,
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

	fn handle_file(&self, absolute_path: &PathBuf, relative_path: &PathBuf, blame: Blame<'_>, lines: &mut Lines) -> Result<()> {
		dbg!(&relative_path);
		if !relative_path.to_string_lossy().to_string().eq("src/main.rs") { // TODO: delete this
			return Ok(());
		}
	
		let file_buffer = fs::read(absolute_path)?;
		let file_string = from_utf8(&file_buffer)?;
		let todo_lines: Vec<(usize, &str)> = file_string
			.lines()
			.enumerate()
			.filter_map(|(index, line)| {
				if self.todo_regex.is_match(line) {
					Some((index + 1, line))
				} else {
					None
				}
			})
			.collect();
		
		if todo_lines.is_empty() {
			return Ok(());
		}
	
		let blame = blame.blame_buffer(&file_buffer)?;
		for (line_number, line) in todo_lines.into_iter() {
			let blame_hunk = blame
				.get_line(line_number)
				.expect("line_number must be valid at this point");
			// println!("======\nline_number={line_number}\tfinal_start_line={}\torg_start_line={}\tlines_in_hunk={}",
			// 	blame_hunk.final_start_line(), blame_hunk.orig_start_line(),
			// 	blame_hunk.lines_in_hunk() - 1,
			// ); // todo: delete this
			if blame_hunk.orig_start_line() == 0 {
				lines.push_uncommitted(line, line_number, relative_path);
			} else {
				lines.push_committed(&blame_hunk, line, line_number, relative_path);
			}
		}
		Ok(())
	}
}

/*fn iter_recursively_all_files(absolute_path: PathBuf, mut relative_path: PathBuf, repo: &Repository) -> Result<()> {
	for entry in fs::read_dir(absolute_path)? {
		let entry = entry?;
		let file_type = entry.file_type()?;
		if file_type.is_file() {
			relative_path.push(Path::new(&entry.file_name()));
			match repo.blame_file(&relative_path, Some(BlameOptions::new()
				.track_copies_any_commit_copies(true)
				.track_copies_same_commit_copies(true)
				.track_copies_same_commit_moves(true)
				.track_copies_same_file(true)
			)) {
				Ok(blame) => handle_blame(&entry.path(), &relative_path, repo, blame)?,
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
}*/

fn main() -> Result<()> {
	let shimi = Foo::new(env::current_dir()?)?;
	shimi.start()?;
	// iter_recursively_all_files(cwd, Path::new("").to_path_buf(), &repo)?;
	Ok(())
}
// todo
