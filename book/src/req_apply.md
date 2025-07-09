# Functional Requirements

As seen in the previous [Context](context.html) chapter, you can require any type to be part of an effect's environment.

You can also require functions of some type to be present and use them in your program descriptions without knowing their exact implementation.

For example, suppose we require a function from `usize` to `usize` and then apply it to a value:

```rust
// Type alias for a function requirement
type FnReq = Fx<fn(usize) -> usize, usize>;

fn double(n: usize) -> usize { n * 2 }

// The effect requires a function of type fn(usize) -> usize
let effect: FnReq = Fx::require();
// Provide the function implementation as the requirement
let applied = effect.provide(double);
// Provide the input value to the function
let result = applied.provide(12).eval();
assert_eq!(result, 24); // result: usize
```

For more complex cases, you can nest effect requests and flatten them using combinators, as in the fx-rs API.
