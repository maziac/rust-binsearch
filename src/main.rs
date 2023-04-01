use std::{env};




fn main() {
    let mut offs = 0;
    let mut size = 0;

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
            args_offs();
        }
        else if arg == "--reloffs" {
            args_reloffs();
        }
        else if arg == "--size" {
            let s = get_next(args_iter, "Expected a size.");
            println!("size: {}", s);

            size = s.parse::<i32>().unwrap();
            //dump(offs, size);
            offs += size;
            args_size();
        }
        else if arg == "--search" {
            args_search();
        }
        else if arg == "--format" {
            args_format();
        }
        else {
            abort(&["Unknown argument: ", arg].concat());
        }
    }
}


fn abort(error_msg: &str) {
    println!("Aborting: {}", error_msg);
    std::process::exit(1);
}


fn get_next(iter: impl Iterator<Item=&'a String>, error_msg: &str) { // -> &str {
    let item = iter.next();
    if item == None {
        abort(error_msg);
    }
 //   item.unwrap()
}



fn args_help() {
    println!("Usage:");

    println!("--help: Prints this help.");
    println!("--offs offset: Offset from start of file. Moves last position.");
    println!("--reloffs offset: Offset from last position. Moves last position.");
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

fn args_size() {
}

fn args_search() {
}

fn args_format() {
}
