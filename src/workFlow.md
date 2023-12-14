# Process of Vault commit command (Incomplete)

1. Let's take an Example directory `dir1`
2. When user types, `vault commit`, we would loop around the directory and create a Blob for every File and create a Tree for every directory..

### How will the Blob be created?

1. Whenever we encounter a file, we would call `make_blob` function, this will create a temp file of the content...

```
blob content_size\0file_content
```

2. This temp file contents would be passed to any `SHA256` hasher, which apparently will give us
