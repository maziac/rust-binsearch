//! Loads a file into a buffer and operates on it.
//! Searches the buffer and dumps out data to stdio from a
//! specific offset and for a specific width.

use std::io::{self, Write};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;


/// Reads a binary file and puts the contents into 'buffer'.
/// # Arguments
/// * 'buffer' - The file contents is appended to the buffer.
/// * 'spath' - The path to the file.
pub fn append_file(buffer: &mut Vec<u8>, spath: &str) {
    // Open file
    let file = match File::open(&spath) {
        Err(why) => panic!("Couldn't open {}: {}", spath, why),
        Ok(file) => file,
    };

    // Read and append bytes
    let mut reader = BufReader::new(file);

    // Read file into vector.
    match reader.read_to_end(buffer) {
        Err(why) => panic!("Couldn't read {}: {}", spath, why),
        Ok(_size) => (),
    };
}


/// Dumps out the contents of 'buffer' to stdout.
/// # Arguments
/// * 'buffer' - The buffer to dump.
/// * 'offset' - The first byte to dump out.
/// * 'size' - The number of bytes to dump out.
pub fn dump(buffer: & Vec<u8>, offset: usize, size: usize) {
    let len = buffer.len();
    if offset < len {
        let mut end = offset + size;
        if end > len {
            end = len;
        }
        io::stdout().write_all(&buffer[offset..end]).unwrap();
    }
}
