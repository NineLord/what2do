use std::{
	time::SystemTime,
	ffi::OsString,
	path::Path,
};

#[derive(Debug)]
pub struct LineData {
	when: SystemTime,
	line: String,
	line_number: usize,
	file_path: OsString,
}

impl LineData {
	pub fn new_everything(when: SystemTime, line: &str, line_number: usize, file_path: &Path) -> Self {
		Self {
			when,
			line: String::from(line),
			line_number,
			file_path: file_path.to_path_buf().into_os_string(),
		}
	}
}