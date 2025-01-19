use clap::{Parser as p, Subcommand};
use std::{env, fs, path::PathBuf, process::exit};
use xta::{scanner::Scanner, token::TokenKind, Parser, XtaError};

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
                eprintln!("Cannot read a directory.");
                exit(1);
            }

            match fs::read_to_string(path) {
                Ok(content) => {
                    let mut scanner = Scanner::new(&content);
                    let mut parser = Parser::new(scanner);
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
