# Core Effects

The list of effects provided by `fx-rs` will grow as new needs are discovered in Rust programs.

Current ideas for contributions:

- Resource management (safe acquire/use/release of resources)
- Structured concurrency (beyond manual mutex/channels)
- Any other effectful pattern useful in Rust—issues and PRs are welcome!

```rust
// Type alias for a function effect
type StringToUsize = Fx<String, usize>;

fn length_of_string(s: String) -> usize { s.len() }

let effect: StringToUsize = Fx::func(length_of_string);
let requirement = "Hello World".to_owned();
let provided = effect.provide(requirement.clone());
let result: usize = provided.eval();
assert_eq!(result, requirement.len()); // result: usize
```
