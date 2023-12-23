# Vault
Vault will be a command line tool (if successful) similar to git which would have multiple features like brances etc etc. 
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

### Contributing 
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
