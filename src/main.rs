//This is th documentation for the std::fs module
//https://doc.rust-lang.org/stable/std/fs/index.html
mod init;
use crate::init::init;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "add" {
            println!("Adding Files to current Branch");
        } else if args[1] == "create" {
            if args.len() > 2 {
                println!("Creating a Branch: {}", args[2]);
            } else {
                println!("Usage: my_tool create <branch_name>");
            }
        } else if args[1] == "delete" {
            println!("Deleting a Branch");
        } else if args[1] == "init" {
            init();
            println!("Init");
        } else {
            println!("Unknown command.");
        }
    } else {
        println!("These are some common vault commands used in various situations: 
1) vault init: Initialize a new vault
2) vault add: Add files to current branch
3) vault create <branch_name>: Create a new branch
4) vault delete: Delete a branch");
    }
}
