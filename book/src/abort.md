# Abort

`Abort[V, E](E)` aborts a computation of `V` with `E`.

When aborted, a computation is *halted*, meaning that any `Map` or `FlatMap` operation over it will not be executed.

The `AbortHandler` replaces aborted effects (those computations where Abort was called) with a `Result` with `E`. Otherwise if the effect was never aborted, a `Result` with `V` is returned. 


- Implementation 
  - [abort.go](https://github.com/vic/fx.go/blob/main/abort/abort.go)
  - [result.go](https://github.com/vic/fx.go/blob/main/abort/result.go)
- Tests [abort_test.go](https://github.com/vic/fx.go/blob/main/abort/abort_test.go)
