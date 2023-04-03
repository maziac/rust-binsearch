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


mod loop_arguments;


use crate::loop_arguments::*;



/// Parses the command line arguments.
/// Reads in file(s) searches and dumps out data.
fn main() {
    // Parse all arguments
    loop_arguments(env::args().collect(), &mut std::io::stdout())
}
