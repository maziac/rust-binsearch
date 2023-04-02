//! Reads the command line arguments and provides functions
//! to access them.


use std::{env};

pub struct Arguments {
	args: Vec<String>,
	index: usize
}

impl Arguments {

	/// Constructor.
	pub fn new() -> Self {
    	let args: Vec<String> = env::args().collect();
		Self {
			args,
			index: 0
		}
	}


	/// Returns the next argument.
	/// If it does nto exist, None is returned.
	pub fn get_next(&mut self) -> Option<&String> {
		self.index += 1;
		if self.index >= self.args.len() {
			None
		}
		else {
			Some(& self.args[self.index])
		}
	}


	/// Returns the next argument and checks if argument exists.
	/// If not the program aborts.
	/// If it exists the argument is unwrapped into a string and returned.
	/// # Arguments
	/// * 'error_msg' - The error message to show if argument does not exist.
	/// # Returns
	/// The argument as a string.
	pub fn get_next_check(&mut self, error_msg: &str) -> String {
		let arg = self.get_next();
		if arg == None {
			panic!("{}", error_msg);
		}
		else {
			arg.unwrap().to_string()
		}
	}
}
