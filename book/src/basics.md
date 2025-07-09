# Basic Effects in `fx-rs`

This section expands on [Concepts](concepts.html) and shows how they relate to the `fx-rs` API, providing intuition on effect requirements and evaluation in Rust.

## `Fx<(), V>`: Immediate Effects

The most basic effects are *immediate* effects. These are of type `Fx<(), V>`, meaning they have no ability requirements and evaluate to a value `V`.

Immediate effects are created using `Fx::pure(value)` or `Fx::value(value)`.

A pure effect just holds an already known value instead of computing it.

The value can be retrieved by calling `.eval()` on the effect. Only effects with no requirements (`()`) can be evaluated directly.

```rust
use fx::Fx;

// Type alias for a simple effect with no requirements
type PureString = Fx<(), String>;

let v = "Hello World".to_owned();
let effect: PureString = Fx::pure(v.clone());
let result: String = effect.eval();
assert_eq!(result, v); // result: String
}
```

## `Fx<S, V>`: Pending Effects

An effect `Fx<S, V>` where `S` is not `()` is a pending effect that needs `S` to be provided before computing `V`.

The most basic pending computation is a function. For example, a function from `String` to `usize` can be expressed as an effect of type `Fx<String, usize>`:

```rust
use fx::Fx;

fn length_of_string(s: String) -> usize {
    s.len()
}

fn func_example() {
    let effect: Fx<String, usize> = Fx::func(length_of_string);
    let requirement = "Hello World".to_owned();
    let provided = effect.provide(requirement.clone());
    let result = provided.eval();
    assert_eq!(result, requirement.len());
}
```

- `Fx::func(f)` produces a pending effect of type `Fx<S, V>` from a function `f: S -> V`.
- `.provide(value)` discharges the requirement and returns `Fx<(), V>`. No computation is performed until `.eval()` is called.
- `.eval()` performs the computation, since all requirements have been provided.

These are the most basic effects in `fx-rs`. More interesting effects are presented in later chapters.
