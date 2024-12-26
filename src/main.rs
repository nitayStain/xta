use clap::{Parser, Subcommand};
use std::{env, fs, path::PathBuf, process::exit};
use xta::{scanner::Scanner, token::Token, XtaError};

#[derive(Parser)]
#[command(name = "Xta Compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        path: PathBuf,
    },
    // Help,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { path } => {
            if !path.is_file() {
                eprintln!("Cannot read a directory.");
                exit(1);
            }

            match fs::read_to_string(path) {
                Ok(content) => {
                    let mut scanner = Scanner::new(&content);

                    while let Some(token) = scanner.next() {
                        println!("{:?}", token);
                        if token == Token::EOF {
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to open file: {}", e);
                }
            }
        }
    }
}
