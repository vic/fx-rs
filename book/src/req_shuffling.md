# Shuffling `And`ed Requirements

In `fx-rs`, requirements are identified by their type, not by name or position. Effect requirements can be freely rearranged with no impact on program meaning.

## `And` Combinators

Several combinators help you rearrange and manipulate `And`ed effect requirements:

```rust
use fx::Fx;

// Type alias for collapsed requirements
type Collapsed = Fx<u32, u32>;

let fx: Fx<(u32, u32), u32> = Fx::immediate((10u32, 10u32), 20u32);
let fx2: Collapsed = fx.and_collapse::<u32>();
let result: u32 = fx2.provide(10).eval();
assert_eq!(result, 20); // result: u32

// Add a Nil (unit) requirement
let fx = Fx::immediate(21u32, 42u32);
let fx2 = fx.and_nil::<(u32, ())>();
let result = fx2.provide((21, ())).eval();
assert_eq!(result, 42);

// Swap requirements
let fx = Fx::immediate((5u8, 10u16), 15u16);
let fx2 = fx.and_swap::<u8, u16, (u16, u8)>();
let result = fx2.provide((10u16, 5u8)).eval();
assert_eq!(result, 15);

// Nest and flatten requirements
let fx = Fx::immediate((3u8, 7u16), 21u16);
let nested = fx.and_nest::<u8, u16>();
let inner = nested.provide(3u8).eval();
let result = inner.provide(7u16).eval();
assert_eq!(result, 21);
let flat = fx.and_nest::<u8, u16>().and_flat::<(u8, u16)>();
let result2 = flat.provide((4u8, 6u16)).eval();
assert_eq!(result2, 21);
```
