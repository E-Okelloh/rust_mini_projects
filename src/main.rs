use std::env;
use std::fs;
use std::process;

fn main() {
    //env ::args() returns an iterator over command-line arguments.
    //Arg 0 is the program name; actual args starts at 1.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wordcount <file>");
        process::exit(1);

    }

    let path = &args[1];

    // Read entire file - fs::read_to_string returns Result<String, io::Error>
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Could not read {}: {}", path, err);
        process::exit(1);
    });

    // split_whitespace iterates over non-empty whitespace-separated segments
    let word_count = contents.split_whitespace().count();
    let line_count = contents.lines().count();
    let byte_count = contents.len();


    println!("{:>8} lines", lines_count); //right-align width 8
    println!("{:>8} words",word_count);
    println!("{:>8} bytes", bytes_count)

}