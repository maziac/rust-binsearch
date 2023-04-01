use std::{env};
use std::io::{self, Write};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;



fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file_name: String;
    let mut offs = 0;
    let mut size: i32;

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();

    // Skip path
    args_iter.next();

    // Parse command line arguments
    loop {
        // Get next argument
        let arg_option = args_iter.next();
        if arg_option == None {
            break
        }
        let arg = arg_option.unwrap();

        // Check argument
        if arg == "--help" {
             args_help();
        }
        else if arg == "--offs" {
            let o = get_next(args_iter.next(), "Expected a size.");
            println!("offs: {}", o);
            offs = o.parse::<i32>().unwrap();
        }
        else if arg == "--roffs" {
            args_reloffs();
        }
        else if arg == "--size" {
            let s = get_next(args_iter.next(), "Expected a size.");
            println!("size: {}", s);
            size = s.parse::<i32>().unwrap();
            dump(&buffer, offs, size);
            offs += size;
        }
        else if arg == "--search" {
            args_search();
        }
        else if arg == "--format" {
            args_format();
        }
        else {
            // It is the filename. Open file.
            append_file(&mut buffer, arg);


            //abort(&["Unknown argument: ", arg].concat());
        }
    }
}



fn append_file(buffer: &mut Vec<u8>, spath: &str) {
    // let path = Path::new(spath);
    // let file = match File::open(&path) {
    // Err(why)=>panic("couldn't open {}: {}", path.display(), why),
    //     Ok(file) => file,
    // };

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


fn dump(buffer: & Vec<u8>, offset:i32, size:i32) {
    io::stdout().write_all(buffer).unwrap();
}



fn abort(error_msg: &str) {
    println!("Aborting: {}", error_msg);
    std::process::exit(1);
}


fn get_next<'a>(arg_option: Option<&'a String>, error_msg: &str) -> &'a String {
    if arg_option == None {
        abort(error_msg);
    }
    arg_option.unwrap()
}



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
    println!("- \"binsearch --search 'abc' --size 10\": Outputs 10 bytes from the first occurence of 'abc'. If not fould nothing is output.");
}


fn args_offs() {
}

fn args_reloffs() {
}

fn args_search() {
}

fn args_format() {
}
