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

use std::{env};

mod bin_dumper;
mod arguments;

use crate::bin_dumper::*;
use crate::arguments::*;



/// Parses the command line arguments.
/// Reads in file(s) searches and dumps out data.
fn main() {
    let mut offs: i32 = 0;
    let mut args = Arguments::new(env::args().collect());
    let mut bin_dumper = BinDumper::new();

    // Parse command line arguments
    while let Some(arg) = args.get_next() {
        // Check argument
        if arg == "--help" {
             args_help();
        }
        else if arg == "--offs" {
            let o = args.get_next_check("Expected an offset.");
            println!("offs: {}", o);
            offs = o.parse::<i32>().unwrap();
        }
        else if arg == "--roffs" {
            let ro = args.get_next_check("Expected a relative offset.");
            println!("roffs: {}", ro);
            offs += ro.parse::<i32>().unwrap();
        }
        else if arg == "--size" {
            let s = args.get_next_check("Expected a size.");
            println!("size: {}", s);
            // Check for max
            let size: i32;
            if s == "all" {
                size = std::i32::MAX;
            }
            else {
                size = s.parse::<i32>().unwrap();
            }
            bin_dumper.dump(offs, size);
            offs += size;
        }
        else if arg == "--search" {
            //args_search();
        }
        else if arg == "--format" {
            //args_format();
        }
        else {
            // It is the filename. Open file.
            bin_dumper.read_file(arg);
        }
    }
}


/// Prints the help.
fn args_help() {
    println!("Usage:");

    println!("--help: Prints this help.");
    println!("--offs offset: Offset from start of file. Moves last position.");
    println!("--roffs offset: Offset from last position. Moves last position.");
    println!("--size size: The number of bytes to evaluate. Moves last position.");
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
