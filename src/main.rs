//This is th documentation for the std::fs module
//https://doc.rust-lang.org/stable/std/fs/index.html
mod commands;
mod core;
mod file_system;
mod hash;
mod utils;
use crate::commands::init::init;
use clap::{Parser, Subcommand};
use commands::commit::commit;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]

struct CLI {
    #[command(subcommand)]
    command: Arguments,
}

#[derive(Subcommand)]
enum Arguments {
    /// Initialize a new vault
    Init,
    /// Commit files to current branch
    Commit,
    // Create a new branch with given name
    // Create {branch_name: String},
    // Switch to given branch name
    // Switch {branch_name: String},
    // Deletes the given branch
    // Delete {branch_name: String},
}


fn main() {
    let cli: CLI = CLI::parse();
    if let Ok(current_dir) = env::current_dir() {
        let _ = match &cli.command {
            Arguments::Init => init(),
            Arguments::Commit => commit(&current_dir).unwrap(),
            // Arguments::Create {branch_name} => create(&branch_name),
            // Arguments::Switch {branch_name} => switch(&branch_name).unwrap(),
            // Arguments::Delete {branch_name} => delete(&branch_name),
        };
    } else {
        eprintln!("Failed to get the current working directory");
    }
}

