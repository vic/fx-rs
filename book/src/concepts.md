# Concepts Tour

This section introduces the concepts of _Effects_, _Abilities_, and _Handlers_ as present in `fx-rs`.

No prior experience with other effect systems is required. We explain concepts from simple to advanced, using Rust idioms.

## Effects

An *Effect* (`Fx<S, V>`, read: "V given S") is a _description_ of a program that computes `V`, provided that the requirement `S` is present.

Effects are descriptions of programs; they compute nothing and produce no side-effects until they are evaluated, once all requirements are met.

A common analogy is a recipe: you have a description of steps and a list of requirements (ingredients and utensils). Once you have them all, you can perform the recipe.

## Abilities

In `Fx<S, V>`, `S` is the *Ability* (sometimes called the set of Abilities, Capabilities, or Effect Environment) needed to compute `V`.

Abilities describe the external resources or side-effects possible while computing `V`.

Examples:

- network abilities (e.g., HTTP requests)
- console abilities (e.g., printing, reading input)
- non-deterministic abilities (e.g., random numbers)
- resource handling (e.g., managing shared resources)
- exception handling (e.g., interruption, finalizers)
- anything else that interacts with the outside world

## Handlers

A *Handler* for an ability is a particular _interpretation_ of what that ability means.

Handlers are the only side-effectful part of your programs. You can have different handlers for the same ability, and each handler decides how and when to perform world-modifying side-effects.

For example, for an HTTP ability, you can have a test handler that mocks responses for tests, or a live handler that performs real network requests in production.
