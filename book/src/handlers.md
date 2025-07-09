# Handlers

A _Handler_ is an effect transformation function of type `impl Handler<'f, R, S, U, V>` (see the `Handler` trait in fx-rs).

Handlers can change effect requirements, typically reducing them, but may also introduce new requirements or change the result type.

## Handling an Effect

Let's rewrite the "length of string" function as a handler in Rust:

```rust
use fx::Fx;
use fx::Handler;

// Type alias for handler effect
type RWHandler = Fx<Env, Output>;

// Handler that takes Fx<(LenFn, ()), usize> and returns Fx<(), usize>
let len_handler: RWHandler = |fx: Fx<'static, (fn(String) -> Fx<'static, (), usize>, ()), usize>| {
    fx.provide_left(|s: String| Fx::pure(s.len()))
};

let effect: Fx<'static, (fn(String) -> Fx<'static, (), usize>, ()), usize> = Fx::func(|s: String| Fx::pure(s.len()));
let handled: Fx<'static, (), usize> = len_handler.handle(effect);
let result = handled.eval();
assert_eq!(result, "hello".len());
```

## Requesting Handlers from the Environment

You can also request that a handler be present as a requirement. This way, the handler is provided once and can be applied anywhere in the program.

```rust
use fx::Fx;
use fx::Handler;

// Type alias for handler effect
type RWHandler = Fx<Env, Output>;

let len_handler: RWHandler = |fx: Fx<'static, (fn(String) -> Fx<'static, (), usize>, ()), usize>| {
    fx.provide_left(|s: String| Fx::pure(s.len()))
};

let effect: Fx<'static, (fn(String) -> Fx<'static, (), usize>, ()), usize> = Fx::func(|s: String| Fx::pure(s.len()));
let provided = effect.provide_left(len_handler);
let result = provided.eval();
assert_eq!(result, "hello".len());
```

Handlers in fx-rs are just values and can be passed, composed, or swapped as needed.

A **Handler** in fx-rs is a transformation: it takes an input (often an effectful request or ability) and produces a new effect. Conceptually, a handler is a function that interprets or transforms effects, often by providing implementations for abilities or by composing/rewriting effects. See the comment in `handler.rs` for details.

An **Ability** is a trait or type that represents a capability or effectful operation. In fx-rs, an ability is conceptually a function of the form `I => Fx<S, O>`, meaning it takes an input `I` and returns an effectful computation producing an output `O` and possibly requiring further abilities `S`. See the comment in `ability.rs` for the canonical definition.
