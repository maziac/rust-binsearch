#![warn(missing_docs)]
//! A binary search and dump utility.
//!
//! With this command line tool you can search byte sequences or strings
//! in a binary file.
//! The found sequences can be piped into another process.
//! It is also possible to simply dump binary data from a certain offset or
//! concatenate several binary files.
//! And you can mix all these functionality.
//!
//! # Examples:
//!
//! ~~~
//! binsearch file.bin --search "abc" --size 5
//! Outputs: abcde
//! For file.bin: xyzabcdeqrst
//! ~~~
//!
//! ~~~
//! binsearch file.bin --offs 2 --size 4
//! Outputs: zabc
//! For file.bin: xyzabcdeqrst
//! ~~~
//!
//! ~~~
//! binsearch file1.bin file2.bin
//! Outputs: abcuvwxyz
//! For file1.bin: abc
//! and file2.bin: uvwxyz
//! ~~~
//!
//! ~~~
//! binsearch file1.bin --offs 1 --size 1 file2.bin --size 3
//! Outputs: buvw
//! For file1.bin: abc
//! and file2.bin: uvwxyz
//! ~~~


use std::io::{Write};

mod bin_dumper;
mod arguments;

use crate::loop_arguments::bin_dumper::*;
use crate::loop_arguments::arguments::*;



/// Separated the looping over the arguments into another function
/// to allow unit tests.
/// Arguments:
/// * 'args_vec' - A vector of strings with the arguments. First element is not used
/// (contains the path to the executable).
/// * 'output' - The destination to write to, e.g. a File or io::stdout() or a Vec.
pub fn loop_arguments(args_vec: Vec<String>, output: &mut impl Write) {
    let mut offs: i32 = 0;
    let mut bin_dumper = BinDumper::new();
    let mut args = Arguments::new(args_vec);

    // Parse command line arguments
    while let Some(arg) = args.get_next() {
        // Check argument
        if arg == "--help" {
             args_help();
        }
        else if arg == "--offs" {
            let o = args.get_next_check("Expected an offset.");
			if o.starts_with("+") {
	            offs += o[1..].parse::<i32>().unwrap();
			}
			else if o.starts_with("-") {
	            offs -= o[1..].parse::<i32>().unwrap();
			}
			else {
	            offs = o.parse::<i32>().unwrap();
			}
        }
        else if arg == "--size" {
            let s = args.get_next_check("Expected a size.");
            // Check for max
            let size: i32;
            if s == "all" {
                size = std::i32::MAX;
            }
            else {
                size = s.parse::<i32>().unwrap();
            }
            bin_dumper.dump(offs, size, output);
            offs += size;
        }
        else if arg == "--search" {
            let s = args.get_next_check("Expected a string.");
            println!("search: {}", s);
			bin_dumper.search(&mut offs, &s);
        }
        else {
            // It is the filename. Open file.
            bin_dumper.read_file(arg);
			offs = 0;
        }
    }
}


/// Prints the help.
fn args_help() {
    println!("Usage:");

    println!("--help: Prints this help.");
    println!("--offs offset: Offset from start of file. Moves last position. It is possible to use relative offset with the '+' or '-' sign. In that case the value is added to the current offset.");
    println!("--size size: The number of bytes to evaluate. Moves last position (offset:=offset+size).");
    println!("--search token [token ...]: Searches for the first occurrence of tokens. Token can be a decimal of hex number or a string. The search starts at last position.");
    println!("--format format: The output format:");
	println!("  - bin: Binary output. The default.");
	println!("  - text: Textual output. Showing the offset and values in rows.");

    println!("Examples:");
    println!("- \"binsearch --offs 10 --size 100\": Outputs the bytes from position 10 to 109.");
    println!("- \"binsearch --offs 10 --size 100 --offs 200 --size 10\": Outputs the bytes from position 10 to 109, directly followed by 200 to 209.");
    println!("- \"binsearch --offs 10 --size 100 --reloffs 10 --size 20\": Outputs the bytes from position 10 to 109, directly followed by 120 to 129.");
    println!("- \"binsearch --search 'abc' --size 10\": Outputs 10 bytes from the first occurrence of 'abc'. If not fould nothing is output.");
}


#[cfg(test)]
mod tests {
    use super::loop_arguments;

    #[test]
    fn loop_arguments_empty() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec!["".to_string()], &mut buf);
			assert_eq!(buf.len(), 0);
		}
	}

    #[test]
    fn loop_arguments_just_filename() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				], &mut buf);
			assert_eq!(buf.len(), 0);
		}
	}

    #[test]
    fn loop_arguments_dump_offs() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "3".to_string(),
				"--size".to_string(), "4".to_string(),
				], &mut buf);
			assert_eq!(buf, "defg".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_2_slices() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "2".to_string(),
				"--size".to_string(), "3".to_string(),
				"--size".to_string(), "4".to_string(),
				], &mut buf);
			assert_eq!(buf, "cdefghi".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_rel_offs() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "+1".to_string(),
				"--size".to_string(), "3".to_string(),
				"--offs".to_string(), "+4".to_string(),
				"--size".to_string(), "2".to_string(),
				], &mut buf);
			assert_eq!(buf, "bcdij".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_out_of_range_1() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "-1".to_string(),
				"--size".to_string(), "3".to_string(),
				], &mut buf);
			assert_eq!(buf, "ab".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_out_of_range_2() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "11".to_string(),
				"--size".to_string(), "3".to_string(),
				], &mut buf);
			assert_eq!(buf, "l".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_out_of_range_4() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "-3".to_string(),
				"--size".to_string(), "3".to_string(),
				], &mut buf);
			assert_eq!(buf, "".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_out_of_range_5() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "12".to_string(),
				"--size".to_string(), "1".to_string(),
				], &mut buf);
			assert_eq!(buf, "".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_out_of_range_6() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "-2".to_string(),
				"--size".to_string(), "20".to_string(),
				], &mut buf);
			assert_eq!(buf, "abcdefghijkl".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_two_files() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--offs".to_string(), "5".to_string(),
				"--size".to_string(), "2".to_string(),
				"test_data/mnopqrstuvwx.bin".to_string(),
				"--offs".to_string(), "1".to_string(),
				"--size".to_string(), "4".to_string(),
				], &mut buf);
			assert_eq!(buf, "fgnopq".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_1() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--search".to_string(), "a".to_string(),
				"--size".to_string(), "2".to_string(),
				], &mut buf);
			assert_eq!(buf, "ab".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_2() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--search".to_string(), "bcd".to_string(),
				"--size".to_string(), "2".to_string(),
				], &mut buf);
			assert_eq!(buf, "bc".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_3() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--search".to_string(), "kl".to_string(),
				"--size".to_string(), "5".to_string(),
				], &mut buf);
			assert_eq!(buf, "kl".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_4() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--search".to_string(), "".to_string(),
				"--size".to_string(), "2".to_string(),
				], &mut buf);
			assert_eq!(buf, "ab".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_5() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdefghijkl.bin".to_string(),
				"--search".to_string(), "xy".to_string(),
				"--size".to_string(), "2".to_string(),
				], &mut buf);
			assert_eq!(buf, "".as_bytes());
		}
	}

    #[test]
    fn loop_arguments_search_6() {
		{
        	let mut buf: Vec<u8> = Vec::new();
			loop_arguments(vec![
				"".to_string(),
				"test_data/abcdabcdaxyz.bin".to_string(),
				"--search".to_string(), "a".to_string(),
				"--offs".to_string(), "+1".to_string(),
				"--search".to_string(), "a".to_string(),
				"--offs".to_string(), "+1".to_string(),
				"--search".to_string(), "a".to_string(),
				"--offs".to_string(), "+1".to_string(),
				"--size".to_string(), "3".to_string(),
				], &mut buf);
			assert_eq!(buf, "xyz".as_bytes());
		}
	}
}
