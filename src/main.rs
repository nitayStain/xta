use std::{env, fs};

use xta::{scanner::Scanner, XtaError};

fn read_file(file_path: &str) -> Result<String, XtaError> {
    fs::read_to_string(file_path).map_err(|_| XtaError::FileError(file_path.to_string()))
}

fn help() {
    eprintln!("Usage: <compiler> <file path>");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }

    let file_path = &args[1];

    let file_content = match read_file(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{}", err);
            help();
            String::from("")
        }
    };

    let mut scanner = Scanner::new(&file_content);
    println!("{:?}", scanner.tokenize().unwrap());
}
