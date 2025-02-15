use std::{
	time::{SystemTime, UNIX_EPOCH, Duration},
	path::Path,
	io::{self, Write},
};
use git2::BlameHunk;
use chrono::{ DateTime, Local };
use state_shift::{impl_state, require, switch_to, type_state};
use super::line_data::LineData;

#[type_state(
    states = (Initial, Sorted),
    slots = (Initial)
)]
pub struct Lines {
	lines: Vec<LineData>,
	now: Option<SystemTime>,
}

#[impl_state]
impl Lines {
    #[require(Initial)]
    pub fn new() -> Lines {
        Lines {
			lines: vec![],
			now: Some(SystemTime::now()),
        }
    }

	#[require(Initial)]
	pub fn push_committed(mut self, when: &BlameHunk<'_>, line: &str, line_number: usize, file_path: &Path) -> Lines {
		self.lines.push(LineData::new_everything(
			UNIX_EPOCH + Duration::from_secs(when.final_signature().when().seconds() as u64),
			line, line_number, file_path
		));
		self
	}

	#[require(Initial)]
	pub fn push_uncommitted(mut self, line: &str, line_number: usize, file_path: &Path) -> Lines {
		self.lines.push(LineData::new_everything(self.now.expect("Initial state has 'now' field"), line, line_number, file_path));
		self
	}

	#[require(Initial)]
	#[switch_to(Sorted)]
	pub fn sort(mut self) -> Lines {
		self.lines.sort_by_key(|line_data| line_data.when());
		Lines {
			lines: self.lines,
			now: None,
		}
	}

	#[require(Sorted)]
	pub fn print(self) -> Lines {
		let mut stdout = io::stdout().lock();
		for line_data in self.lines.iter() {
			let date_time: DateTime<Local> = line_data.when().into();
			writeln!(stdout, "{}\t{}::{}\t{}",
				date_time.format("%Y-%m-%d %H:%M:%S %z"),
				line_data.file_path().to_string_lossy(),
				line_data.line_number(),
				line_data.line().trim()
			).unwrap();
		}
		drop(stdout);
		self
	}
}
