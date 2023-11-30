# Vault
Vault will be a command line tool (if successful) similar to git which would have multiple features like brances etc etc. 
```
__     __          _ _   
\ \   / /_ _ _   _| | |_ 
 \ \ / / _` | | | | | __|
  \ V / (_| | |_| | | |_ 
   \_/ \__,_|\__,_|_|\__| , simplified version control for local files 
```

# Info
This is my try at learning Rust. I literally even had to ask ChatGPT on how to start a new Cargo project while starting this : ) . I don't even know if I would even be able to pull this off. Lets hope for the best. 

# Idea 
So basically, something like `vault init` should initialize a vault repository in the current working directory. Then comes the actual git part - 

1) `vault init` would initially start a `main` branch in the `.vault` folder. This would create something like a `main` directory inside `./vault`.
2) Every file in the present working directory would be present in a directory inside `main` (branch) maybe by a encoded name to prevent collissions.
3) Whenever user runs `vault add` in the present working directory, would be looped, and added to the current branch.
4) These files would be compressed by maybe some library or algorithm to keep the project memory efficient.
5) If a user creates a new branch by say `vault create {branch_name}` , a new folder with the branch name would be created alongside the `main` folder depicting the new branch.
6) By default, whenever a new branch is created, the contents of the current branch should be copied over to the new branch.
7) Each branch should be independent of each other, and whenever the user switches branches by say `vault switch {branch_name}` all the files would be overwritten according to the new branch.
8) MAYBE LATER IF WE REACH TILL HERE , we can add the functionality of deleting a branch, merging two branches etc.

Hmm :/, Enough talking, lets start!
# Set Up Locally 
1) Clone this repository by running the command `git clone https://github.com/shubhexists/vault`.
2) `cd` into the directory and run `cargo build --release`. This will create binaries for the project.
3) Open a new terminal and open `.bashrc` in your favourite editor, say `nano ~/.bashrc`.
4) Export the path of the executable (It is in the `/target/release/` directory .) For eg, it was

      `export PATH="$PATH:/home/jerry/Desktop/vault/target/release"` for me.

6) Save the file and source the terminal or simple open a new terminal instance.
7) You are not all set to "VAULTIFY" your local files :)

# Todo
Release 1 - This includes all the basic functionalities that are to be added in vault.

- [x] - `vault init`
- [x] - `vault create branch_name`
- [ ] - `vault switch branch_name`
- [ ] - `vault add`
- [ ] - `vault delete branch_name`

Release 2 - This includes the compression of the branches and adding support for non text files also..
- [ ] - Compression using `flate32` crate
- [ ] - Adding support for non text files (.exe.. etc)

Release 3 - Implementing logic for actual version control system
- [ ] - Tracking file changes etc..
