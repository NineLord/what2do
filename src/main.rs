#![allow(unused)]
#![deny(unused_must_use)]

mod script_logic {
	mod result {
		mod line_data;
		pub mod lines;
	}
	pub mod find_what_2_do;
}

use std::env;
use anyhow::Result;
use crate::script_logic::find_what_2_do::FindWhat2Do;


fn main() -> Result<()> {
	let shimi = FindWhat2Do::new(env::current_dir()?)?;
	shimi.start()?;
	Ok(())
}
// todo
