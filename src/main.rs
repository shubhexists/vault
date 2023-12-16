//This is th documentation for the std::fs module
//https://doc.rust-lang.org/stable/std/fs/index.html
mod commands;
mod compress_zlib;
mod core;
mod hash;
mod utils;
use crate::commands::init::init;
use clap::{Parser, Subcommand};
use commands::{commit, create, switch, cat};
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
    /// Create a new branch with given name
    Create { branch_name: String },
    /// Switch to given branch name
    Switch { branch_name: String },
    /// Get the actual data, stored in the ZLib hash
    Cat { hash_string: String }, 
    // Deletes the given branch
    // Delete {branch_name: String},
}

fn main() {
    let cli: CLI = CLI::parse();
    if let Ok(current_dir) = env::current_dir() {
        let _ = match &cli.command {
            Arguments::Init => init(),
            Arguments::Commit => commit(&current_dir).unwrap(),
            Arguments::Create { branch_name } => create(&branch_name),
            Arguments::Switch { branch_name } => switch(&branch_name),
            Arguments::Cat { hash_string } => cat(&hash_string).unwrap(),
            // Arguments::Delete {branch_name} => delete(&branch_name),
        };
    } else {
        eprintln!("Failed to get the current working directory");
    }
}
