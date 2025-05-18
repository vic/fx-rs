# Context Requirements

The `Ctx[V]() Fx[V, V]` function creates an effect that requires some value `V` as part of the environment and evaluates to `V`.

It can be used to request the presence of some services (interfaces or collections of methods that produce related effects) or more generally, an evidence that a value `V` is part of the environment.

In the following example we simply map over an string to compute its length.

```go
var eff Fx[string, int] = Map(Ctx[string](), LengthOfString)
var result int = Eval(Provide(eff, "hello"))
assert(result == len("hello"))
```

Calling the `Const[S](V) Fx[S, V]` is like maping a context over a constant function.
As you might have guessed, `Pure` is defined in terms of `Const`.

```go
var a Fx[string, int] = Map(Ctx[string](), func(_ string) int { return 42 })
var b Fx[string, int] = Const[string](42)

var c Fx[Nil, int] = Pure(22)
var d Fx[Nil, int] = Const[Nil](22)
```
