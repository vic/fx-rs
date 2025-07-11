# Macro-Based Ergonomics in fx-rs

fx-rs uses Rust's macro system to provide ergonomic APIs and reduce boilerplate for effectful programming. Some highlights:

## Ability Derivation

Procedural macros in the `abilities_macro` crate allow you to derive ability traits and handlers automatically, making it easy to define new effectful operations and their interpreters.

## Direct-Style Macros

The `do_macro` and `do_traits` crates provide macros for writing effectful code in a direct, imperative style, reducing the need for manual chaining and improving readability.

### Example: fx_do! with .same() and .bind()

The `fx_do!` macro allows you to write effectful code in a direct style. Under the hood, `.same()` is used for `map_m` (monadic map), and `.bind()` is used for `flat_map` (monadic bind):

```rust
use fx::Fx;
use fx_do::fx_do;

// Type alias for macro-based effect
type MacroEff = Fx<MacroEnv, MacroVal>;

fx_do! {
    let x: MacroEff = Fx::pure(1);
    let y = x.same(); // equivalent to .map_m
    let z = y.bind(); // equivalent to .flat_map
    Fx::pure(z)
}
```

## Lens and Field Macros

The `lens_macro`, `field_macro`, and `forall_macro` crates provide macros for generating lenses and working with generic fields, enabling fine-grained, type-safe state manipulation and modular effect composition.

### field_macro: Field-Based Lenses

The `field_macro` crate provides macros to automatically generate lenses for struct fields, making it easy to focus on and update nested fields in your state.

#### Example: field_macro

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

### forall_macro: Quantified Field Access

The `forall_macro` crate provides macros for working with generic fields and quantified access, enabling advanced patterns for generic and reusable effectful code.

## ContextBuilder Macro: Effectful Context Provision

The `builder_macro` crate provides a procedural macro for generating type-safe, incremental context builders for struct contexts. This enables ergonomic and effectful context provision, especially when working with fx-rs's effect system and trait-based dependency injection.

### Example: Using ContextBuilder for Effectful Contexts

```rust
use builder_macro::ContextBuilder;
use fx::{Has, Put};

#[derive(ContextBuilder, Debug, Clone)]
struct MyContext {
    a: i32,
    b: String,
    c: bool,
}

// Incrementally build the context
let ctx = MyContextBuilder::empty()
    .a(42)
    .b("hello".to_string())
    .c(true)
    .build();

// Use trait-based access
assert_eq!(Has::<i32>::get(ctx.clone()), 42);
assert_eq!(Has::<String>::get(ctx.clone()), "hello");
assert_eq!(Has::<bool>::get(ctx.clone()), true);

// Use macro-generated accessors
assert_eq!(ctx.a(), Some(42));
assert!(ctx.has_a());

// Compose with fx-rs effect system
// (see fx.rs book for more advanced examples)
```

The macro generates:

- A builder type with `put_*` methods for each field
- Marker types for tracking field presence
- `build()` method (requires all fields set)
- Trait impls for `Has` and `Put`
- Field accessors and presence checks

This pattern enables ergonomic, type-safe context provision and mutation for effectful code.

______________________________________________________________________

These macros are designed to work seamlessly with the fx-rs core, making advanced effect patterns accessible and ergonomic for Rust developers.
