# Vault
Vault will be a command line tool (if successful) similar to git which would have multiple features like brances etc etc. 

Drop of a ✨ if you are here. It would mean a lot : )
```
__     __          _ _   
\ \   / /_ _ _   _| | |_ 
 \ \ / / _` | | | | | __|
  \ V / (_| | |_| | | |_ 
   \_/ \__,_|\__,_|_|\__| , simplified version control for local files 
```
### About

Vault is a version control system in Rust, written as a learning project. It follows a similar approach of how Git works i.e.
-  Files are stored as Blobs and Directories as Trees. 
-  SHA256 is used to encode these objects.
-  ZLib Compression is used for maximum efficiency.

For more details, Refer to [Workflow.md](https://github.com/shubhexists/vault/blob/master/src/workFlow.md) (wip)

Vault is not suitable for real-world use, but might be of interest for learning about git-internals.

# Installation 
You can directly install from [Cargo](https://crates.io/crates/vault_vcs) by
```
cargo install vault_vcs
```

### References
- [Git Book](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects)
- [Article](https://dev.to/nopenoshishi/make-your-original-git-analyze-section-139d#de)

### From Source 
1) Clone this repository by running the command
```
git clone https://github.com/shubhexists/vault
```
2) `cd` into the directory and run
```
cargo build --release
```
 This will create binaries for the project.
 
3) Export the path of the executable (It is in the `/target/release/` directory .) For eg,
```
   export PATH="$PATH:/home/jerry/Desktop/vault/target/release"
```
4) You are now all set to "VAULTIFY" your local files :)

# Commands
1) To initialize a new vault instance in your present working directory.
```
vault init
```
2) To create a new commit.
```
vault commit -m "Your Commit Message"
```
Parameter ` -m ` or ` --message ` is optional. It would take an empty message by default if no message is provided.

3) To create a new branch.
```
vault create branch_name
```
4) To revert commits (get back to a previous point of directory)
```
vault revert -l "No. of commits" "dir_name"
```
- Parameter ` -l ` or ` --level ` defines the number of commits to go back.. For eg. -l 2 would go back 2 commits in the repository. If nothing is provided , default vaule would be taken as 1.

- ` dir_name ` requires a directory name in which the files would be added. If nothing is provided, it would replace the files of the current directory.. (wip)

For eg. for `vault revert -l 2 hello`, a new directory would be created namely `hello`, that would have the files 2 commits ago.. 

5) Delete a Branch 
```
vault delete branch_to_delete
```
6) Switching to another branch ( git checkout )
```
vault switch branch_to_switch
```
7) Logs of the current branch 
```
vault log
```
Note - Output of vault log just consists of the logs of current active branch.

![Output of vault log](https://github.com/shubhexists/vault/assets/110319892/49e44032-dbcb-4741-b86d-7ca54a7d8a42)
8) (Mainly For Debugging Purposes) To read the contents of a Zlib compressed binary 
```
vault cat hash_string_to_read
```
Note - `vault cat` command should be run in the root directory. i.e. the directory in which ` .vault ` exists.
![Screenshot from 2023-12-23 10-05-08](https://github.com/shubhexists/vault/assets/110319892/f9907727-d492-4e5f-ac85-83605079a3b1)

# Limitations / WIP
1) ` .vaultignore ` is not functional currently
2) To add - Test cases and subsequent workflows
3) Add more useful commands 

There would be probably many more! Some of them are in the [issues](https://github.com/shubhexists/vault/issues).

# Thanks
If you read till here, thanks for showing interest in the project :)
