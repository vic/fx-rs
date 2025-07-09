# Performance and Trade-offs

fx-rs is designed for flexibility and composability, but also aims for competitive performance. It uses dynamic dispatch for handler values, but leverages Rust's inlining and monomorphization where possible.

## Example: Static vs Dynamic Dispatch

```rust
use fx::Fx;

fn static_add(n: u32) -> u32 { n + 1 }
let fx_static = Fx::func(static_add);
let result = fx_static.provide(41).eval();
assert_eq!(result, 42);

let fx_dyn = Fx::pending(|n: u32| Fx::value(n + 1));
let result = fx_dyn.provide(41).eval();
assert_eq!(result, 42);
```

Use static dispatch for performance-critical code, and dynamic handler values for flexibility and modularity.
