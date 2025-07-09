# Effect Polymorphism and Generic Abstractions

fx-rs enables writing code generic over effects, supporting reusable libraries and higher-order effectful functions.

## Example: Generic Logging

```rust
use fx::Fx;

trait Log { fn log(&self, msg: &str); }

fn with_logging<'f, L: Log>(msg: &'f str) -> Fx<'f, L, ()> {
    Fx::pure(move |l: &L| l.log(msg))
}

// Can be used with any Log implementation
```

This enables scalable, composable effectful code and reusable libraries.
