# Abort

`Abort<V, E>(E)` aborts a computation of type `V` with an error `E`.

When aborted, a computation is halted: any `map` or `flat_map` operation over it will not be executed.

The `AbortHandler` replaces aborted effects (where `Abort` was called) with a `Result<V, E>`. If the effect was never aborted, a `Result<V, E>` is returned.

See the implementation and tests in the fx-rs codebase for details.
