use clap::Parser as p;
use std::{fs, path::PathBuf, process::exit};
use xta_lexer::scanner::Scanner;
use xta_parser::parser::Parser;

#[derive(p)]
#[command(name = "Xta", version, about, long_about = None)]
pub enum Cli {
    Run { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Run { path } => {
            if !path.is_file() {
                eprintln!("~ Error: Cannot read the specified source file.");
                exit(1);
            }
            match fs::read_to_string(path) {
                Ok(content) => {
                    let mut scanner = Scanner::new(&content);
                    let mut parser = Parser::new(scanner);

                    let stmts = parser.parse_file();
                    for stmt in stmts {
                        println!("{:?}", stmt);
                    }

                    for error in parser.errors {
                        eprintln!("{}", error);
                    }
                }
                Err(e) => {
                    println!("Failed to open file: {}", e);
                }
            }
        }
    }
}
