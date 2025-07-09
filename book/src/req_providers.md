# Providing Requirements

Effect requirements can be provided in any order, with no impact on program evaluation.

## Provide Combinators

These functions are used to eliminate requirements from effects. Only when `()` is the only remaining requirement can the effect be evaluated.

```rust
use fx::Fx;

// Type alias for a requirement-providing effect
type TimesTen = Fx<usize, usize>;

let fx: TimesTen = Fx::pending(|n: usize| Fx::value(n * 10));
let fx2 = fx.provide(12);
assert_eq!(fx2.eval(), 120); // result: usize

// Type alias for a pair requirement
type PairReq = Fx<(i32, i32), i32>;

let fx: PairReq = Fx::value(7);
let fx2 = fx.provide_left::<i32, i32>(1);
// ...
```

See the fx-rs API for more combinators and details.
