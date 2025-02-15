use std::{
	time::SystemTime,
	ffi::OsString,
	path::Path,
};
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Debug, CopyGetters, Getters)]
pub struct LineData {
	#[getset(get_copy = "pub")]
	when: SystemTime,
	#[getset(get = "pub")]
	line: String,
	#[getset(get_copy = "pub")]
	line_number: usize,
	#[getset(get = "pub")]
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