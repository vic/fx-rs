# What Makes fx-rs Unique

fx-rs is not just another effect system ported to Rust. It is designed from the ground up to take advantage of Rust's type system, ownership, and module system, while providing a modern, ergonomic, and composable effect API. Here are some of the features that make fx-rs stand out:

## 1. Programmable Handlers as First-Class Values

Handlers in fx-rs are values, not static typeclasses. This means you can pass, compose, and swap handlers at runtime, enabling scoped and modular effect interpretation. Handler replacement is explicit and type-safe, allowing for powerful patterns like test mocking, dependency injection, and context adaptation.

## 2. Trait-Based Abilities and Contexts

Abilities are defined as Rust traits, and the effect context is any Rust struct. This enables named, nested, and multiple abilities, and makes dependency management explicit and ergonomic. You can use Rust's trait system to define effectful operations, and context structs to group and manage abilities.

## 3. Composable Context Adaptation

fx-rs provides combinators like `adapt` and `contra_map` to adapt computations to different contexts. This allows for scoped dependency replacement, modular handler wiring, and seamless integration with existing Rust code.

## 4. Direct-Style and Builder APIs

While fx-rs supports traditional `map`/`flat_map` combinators, it also offers (and is evolving) direct-style macros and builder patterns for more ergonomic effectful code. This reduces boilerplate and makes effectful code look and feel like regular Rust.

## 5. Lenses and Fine-Grained State

State effects in fx-rs are modular and composable, with support for lenses to focus on subparts of state. This enables fine-grained, type-safe state manipulation and modular effect composition.

## 6. Structured Concurrency as Effects

Concurrency primitives (spawning, channels, async tasks) can be modeled as abilities, making concurrent operations explicit, composable, and testable. Handlers can swap between real and mock concurrency backends for robust testing.

## 7. Macro-Based Ergonomics

fx-rs leverages Rust's macro system (see the macro crates in the repository) to reduce boilerplate, derive abilities, and provide ergonomic APIs for effectful programming.
