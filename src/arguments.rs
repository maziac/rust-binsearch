//! Reads the command line arguments and provides functions
//! to access them.



pub struct Arguments {
	args: Vec<String>,
	index: usize
}

impl Arguments {

	/// Constructor.
	pub fn new(args: Vec<String>) -> Self {
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



#[cfg(test)]
mod tests {
    use crate::arguments::Arguments;

    #[test]
    fn get_next_empty() {
		let mut args = Arguments::new(vec!["path".to_string()] );

		// Next is empty
		assert_eq!(args.get_next(), None);
	}

    #[test]
    fn get_next_a_few() {
		let mut args = Arguments::new(vec![
			"path".to_string(),
			"file".to_string(),
			"--offs".to_string(),
			"+100".to_string(),
			] );

		// Next is empty
		assert_eq!(args.get_next().unwrap(), "file");
		assert_eq!(args.get_next().unwrap(), "--offs");
		assert_eq!(args.get_next().unwrap(), "+100");
		assert_eq!(args.get_next(), None);
	}

}
