use std::{
	time::{SystemTime, UNIX_EPOCH, Duration},
	path::Path,
};
use git2::BlameHunk;
use super::line_data::LineData;

pub struct Lines {
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
		// TODO: Calling `when()` here gives `Segmentation fault (core dumped)`
		self.lines.push(LineData::new_everything(
			UNIX_EPOCH + Duration::from_secs(when.final_signature().when().seconds() as u64),
			line, line_number, file_path
		));
	}

	pub fn push_uncommitted(&mut self, line: &str, line_number: usize, file_path: &Path) {
		self.lines.push(LineData::new_everything(self.now, line, line_number, file_path));
	}
}