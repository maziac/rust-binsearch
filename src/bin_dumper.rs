//! Loads a file into a buffer and operates on it.
//! Searches the buffer and dumps out data to stdio from a
//! specific offset and for a specific width.

use std::io::{self, Write};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;


pub struct BinDumper {
    buffer: Vec<u8>
}


impl BinDumper {

	/// Constructor.
	pub fn new() -> Self {
    	let buffer: Vec<u8> = Vec::new();
		Self {
			buffer
		}
	}


	/// Reads a binary file and puts the contents into 'buffer'.
	/// # Arguments
	/// * 'spath' - The path to the file.
	pub fn read_file(&mut self, spath: &str) {
        // Clear buffer
		self.buffer.clear();
		// Open file
		let file = match File::open(&spath) {
			Err(why) => panic!("Couldn't open {}: {}", spath, why),
			Ok(file) => file,
		};

		// Read and append bytes
		let mut reader = BufReader::new(file);

		// Read file into vector.
		match reader.read_to_end(&mut self.buffer) {
			Err(why) => panic!("Couldn't read {}: {}", spath, why),
			Ok(_size) => (),
		};
	}


	/// Dumps out the contents of a slice of 'buffer' to stdout.
	/// # Arguments
	/// * 'offset' - The first byte to dump out.
	/// * 'size' - The number of bytes to dump out.
	pub fn dump(& self, offset: usize, size: usize) {
		let dump_buffer = self.dump_to_buffer(offset, size);
		io::stdout().write_all(dump_buffer).unwrap();
	}


	/// Dumps out the contents of a slice of 'buffer' to a buffer.
	/// # Arguments
	/// * 'offset' - The first byte to dump out.
	/// * 'size' - The number of bytes to dump out.
	fn dump_to_buffer(& self, offset: usize, size: usize) -> &[u8] {
		let len = self.buffer.len();
		let mut start = offset;
		if start >= len {
			start = len-1;
		}
		let mut end = offset + size;
		if end > len {
			end = len;
		}
		&self.buffer[start..end]
	}


}




#[cfg(test)]


mod tests {
    use crate::bin_dumper::BinDumper;

    #[test]
    fn dump_to_buffer() {
		let mut bd = BinDumper::new();
		bd.read_file("test_data/abcdefghijkl.bin");
		let buf = bd.dump_to_buffer(0, std::usize::MAX);
		assert_eq!(buf.len(), 12);
	}
}
