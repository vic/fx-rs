# Context Requirements

The `State::get()` function creates an effect that requires some value `S` as part of the environment and evaluates to `S`.

It can be used to request the presence of services (traits or collections of methods that produce related effects) or, more generally, evidence that a value is part of the environment.

For example, mapping over a string to compute its length:

```rust
use fx::State;

// Type alias for requirement effect
type StringLen = Fx<String, usize>;

let eff: StringLen = State::get::<String>().map(|s| s.len());
let result: usize = eff.provide("hello".to_owned()).eval();
assert_eq!(result, 5); // result: usize
```

Calling `Fx::value` is like mapping a context over a constant function. `Fx::pure` is defined in terms of `Fx::value`.

```rust
use fx::State;
use fx::Fx;

let a = State::get::<String>().map(|_| 42);
let b = Fx::value(42);
let c = Fx::pure(22);
```
