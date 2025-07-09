# Resource Management and Bracket Patterns

fx-rs supports safe, composable resource management using bracket-like APIs and Rust’s ownership model.

## Example: File Handling

```rust
use fx::Fx;

fn with_file<'f>(path: &'f str) -> Fx<'f, (), String> {
    Fx::pure(move |_| format!("opened {}", path))
}

let fx = with_file("foo.txt");
let result = fx.eval();
assert_eq!(result, "opened foo.txt");
```

Resources are acquired, used, and released safely, with automatic cleanup and error handling.
