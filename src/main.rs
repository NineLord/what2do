#![allow(unused)]
#![deny(unused_must_use)]

mod script_logic {
	mod result {
		mod line_data;
		pub mod lines;
	}
	pub mod find_what_to_do;
}

use std::env;
use anyhow::Result;
use crate::script_logic::find_what_to_do::FindWhatToDo;


fn main() -> Result<()> {
	let find_what_to_do = FindWhatToDo::new(env::current_dir()?)?;
	let lines = find_what_to_do.start()?;
	println!("{lines:?}");
	Ok(())
}
// todo
