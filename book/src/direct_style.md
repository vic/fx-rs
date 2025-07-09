# Direct-Style and Builder APIs

fx-rs supports not just monadic chaining but also direct-style macros and builder patterns for ergonomic effectful code.

## Example: Direct-Style Macro with fx_do!

The `fx_do!` macro allows you to write effectful code in a direct, imperative style. Under the hood, `.same()` is used for `map_m` (monadic map), and `.bind()` is used for `flat_map` (monadic bind):

```rust
use fx::Fx;
use fx_do::fx_do;

fx_do! {
    let x = Fx::pure(1);
    let y = x.same(); // equivalent to .map_m
    let z = y.bind(); // equivalent to .flat_map
    Fx::pure(z)
}
```

## Example: Builder Pattern

```rust
use fx::Fx;

let result = Fx::builder()
    .get_value()
    .compute()
    .log()
    .run();
```

These patterns reduce boilerplate and make effectful code look and feel like regular Rust.
