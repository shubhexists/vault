# Vault
Vault will be a command line tool (if successful) similar to git which would have multiple features like brances etc etc. 
```
__     __          _ _   
\ \   / /_ _ _   _| | |_ 
 \ \ / / _` | | | | | __|
  \ V / (_| | |_| | | |_ 
   \_/ \__,_|\__,_|_|\__| , simplified version control for local files 
```
# Releases
See branches for a specific release of vault.

# Set Up Locally 
1) Clone this repository by running the command `git clone https://github.com/shubhexists/vault`.
2) `cd` into the directory and run `cargo build --release`. This will create binaries for the project.
3) Open a new terminal and open `.bashrc` in your favourite editor, say `nano ~/.bashrc`.
4) Export the path of the executable (It is in the `/target/release/` directory .) For eg, it was

      `export PATH="$PATH:/home/jerry/Desktop/vault/target/release"` for me.

6) Save the file and source the terminal or simple open a new terminal instance.
7) You are not all set to "VAULTIFY" your local files :)
