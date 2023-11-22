//This is th documentation for the std::fs module
//https://doc.rust-lang.org/stable/std/fs/index.html

mod branches;
mod init;
mod utils;

use crate::{
    branches::{create, delete, switch},
    init::init,
};

use clap::{Parser, Subcommand};

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
    /// Add files to current branch
    Add,
    /// Create a new branch with given name
    Create {branch_name: String},
    /// Switch to given branch name
    Switch {branch_name: String},
    /// Deletes the given branch
    Delete {branch_name: String},
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Arguments::Init => init(),
        Arguments::Add => println!("Adding Files to current Branch"),
        Arguments::Create {branch_name} => create(&branch_name),
        Arguments::Switch {branch_name} => switch(&branch_name).unwrap(),
        Arguments::Delete {branch_name} => delete(&branch_name),
    }

}
