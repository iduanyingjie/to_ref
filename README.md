Before:

```rust
let url = "www.google.com".to_string();
let path: &Path = url.as_ref();
```

Now:

```rust
let url = "www.google.com".to_string();
let path = url.to_ref::<Path>();
```

