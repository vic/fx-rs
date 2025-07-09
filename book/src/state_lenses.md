# State Effects, Lenses, and Contexts in fx-rs

fx-rs provides modular and composable state effects, with first-class support for lenses, struct-based contexts, and advanced combinators for working with state and context.

## Modular State and Lenses

State effects are not global; you can focus on any part of your context using lenses. This allows you to write effectful code that only depends on the state it needs, improving modularity and testability.

### Example: Using a Lens

```rust
use fx::{State, Lens};

// Type alias for state lens effect
type LensEff = Fx<State, Value>;

#[derive(Clone)]
struct AppState {
    counter: u32,
    user: User,
}

#[derive(Clone)]
struct User {
    name: String,
}

let lens = Lens::<AppState, u32>::new();
let get_counter = State::<AppState>::get().via(lens.zoom_out());
```

## Struct-Based Contexts and Pair

fx-rs supports both tuple and struct-based contexts. The `Pair` trait allows you to use custom structs as effect environments, not just tuples.

### Example: Pair for Struct Context

```rust
use fx::Pair;

#[derive(Clone)]
struct Ctx { a: i32, b: bool }

impl Pair<i32, bool> for Ctx {
    fn fst(self) -> i32 { self.a }
    fn snd(self) -> bool { self.b }
}
```

This enables ergonomic, named, and type-safe contexts for your effects.

## forall: Quantified Effects

The `forall` combinator allows you to abstract over part of the context, making your effect generic over some state or ability.

### Example: forall

```rust
use fx::{Fx, State};

let fx: Fx<'static, i32, i32> = State::<i32>::get();
let fx2 = fx.forall(|fx| fx.map(|n| n + 1));
let result = fx2.provide(41).eval();
assert_eq!(result, 42);
```

## provide_part: Partial Context Provision

`provide_part` allows you to provide only part of a context, leaving the rest to be provided later. This is useful for composing effects with partially known environments.

### Example: provide_part

```rust
use fx::Fx;

let fx: Fx<'static, (i32, bool), i32> = Fx::value(7);
let fx2 = fx.provide_part::<i32, bool>(1); // Now Fx<'static, bool, i32>
```

## field_macro: Field-Based Lenses

The `field_macro` crate provides macros to automatically generate lenses for struct fields, making it easy to focus on and update nested fields in your state.

### Example: field_macro

```rust
use field_macro::Field;

#[derive(Field, Clone)]
struct AppState {
    #[field]
    counter: u32,
    #[field]
    user: User,
}

// Now you can use AppState::counter() and AppState::user() as lenses.
```

______________________________________________________________________

Lenses, struct-based contexts, and advanced combinators make fx-rs a powerful tool for modular, reusable, and testable stateful code.
