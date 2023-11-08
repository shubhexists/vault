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
5) If a user creates a new branch by say `vault create -b {branch_name}` , a new folder with the branch name would be created alongside the `main` folder depicting the new branch.
6) By default, whenever a new branch is created, the contents of the current branch should be copied over to the new branch.
7) Each branch should be independent of each other, and whenever the user switches branches by say `vault checkout {branch_name}` all the files would be overwritten according to the new branch.
8) MAYBE LATER IF WE REACH TILL HERE , we can add the functionality of deleting a branch, merging two branches etc.

Hmm :/, Enough talking, lets start!
