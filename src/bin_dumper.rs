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
	pub fn dump(& self, offset: i32, size: i32) {
		let dump_buffer = self.dump_to_buffer(offset, size);
		io::stdout().write_all(dump_buffer).unwrap();
	}


	/// Dumps out the contents of a slice of 'buffer' to a buffer.
	/// # Arguments
	/// * 'offset' - The first byte to dump out.
	/// * 'size' - The number of bytes to dump out.
	fn dump_to_buffer(& self, offset: i32, size: i32) -> &[u8] {
		let len: i32 = self.buffer.len() as i32;
		let mut start = offset;
		let mut count = size;
		if start >= len {
			start = len-1;
		}
		else if start < 0 {
			count += start;
			start = 0;
		}
		if count > len - start {
			count = len - start;
		}
		let  end = start + count;
		&self.buffer[start as usize..end as usize]
	}


}




#[cfg(test)]


mod tests {
    use crate::bin_dumper::BinDumper;

    #[test]
    fn read_file() {
		let mut bd = BinDumper::new();

		// Read existing file
		bd.read_file("test_data/abcdefghijkl.bin");
		assert_eq!(bd.buffer, "abcdefghijkl".as_bytes());

		// Read empty file
		bd.read_file("test_data/empty.bin");
		assert_eq!(bd.buffer, &[]);
	}


	#[test]
	#[should_panic]
    fn read_file_not_existing() {
		let mut bd = BinDumper::new();

		// Read not existing file
		bd.read_file("test_data/not_existing.bin");
	}


    #[test]
    fn dump_to_buffer() {
		let mut bd = BinDumper::new();
		bd.read_file("test_data/abcdefghijkl.bin");

		// All
		{
			let buf = bd.dump_to_buffer(0, std::i32::MAX);
			assert_eq!(buf.len(), 12);
			assert_eq!(buf, "abcdefghijkl".as_bytes());
		}

		// All
		{
			let buf = bd.dump_to_buffer(0, 12);
			assert_eq!(buf.len(), 12);
			assert_eq!(buf, "abcdefghijkl".as_bytes());
		}

		// Right
		{
			let buf = bd.dump_to_buffer(8, std::i32::MAX);
			assert_eq!(buf.len(), 4);
			assert_eq!(buf, "ijkl".as_bytes());
		}

		// Left
		{
			let buf = bd.dump_to_buffer(-4, 12);
			assert_eq!(buf.len(), 8);
			assert_eq!(buf, "abcdefgh".as_bytes());
		}

		// Partial
		{
			let buf = bd.dump_to_buffer(1, 10);
			assert_eq!(buf.len(), 10);
			assert_eq!(buf, "bcdefghijk".as_bytes());
		}

	}
}
