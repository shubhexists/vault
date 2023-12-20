mod commands;
mod core;
mod utils;
use std::env;

use crate::commands::init::init;
use clap::{Parser, Subcommand};
use commands::delete::delete;
use commands::log::log;
use commands::{cat::cat, commit::commit, create::create, switch::switch};

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
    Commit {
        #[arg(short, long)]
        message: String,
    },
    /// Create a new branch with given name
    Create { branch_name: String },
    /// Switch to given branch name
    Switch { branch_name: String },
    /// Get the actual data, stored in the ZLib hash
    Cat { hash_string: String },
    /// Deletes the given branch, Use `vault delete .` to entirely remove vault from your directory!
    Delete { branch_name: String },
    ///
    Log,
}

fn main() {
    let cli: CLI = CLI::parse();
    if let Ok(current_dir) = env::current_dir() {
        let _ = match &cli.command {
            Arguments::Init => init(),
            Arguments::Commit { message } => commit(&current_dir, message).unwrap(),
            Arguments::Create { branch_name } => create(&branch_name),
            Arguments::Switch { branch_name } => switch(&branch_name),
            Arguments::Cat { hash_string } => cat(&hash_string).unwrap(),
            Arguments::Delete { branch_name } => delete(&branch_name).unwrap(),
            Arguments::Log => log().unwrap(),
        };
    } else {
        eprintln!("Failed to get the current working directory");
    }
}
