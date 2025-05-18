# Basic Effects on `Fx.go`

This section expands on [Concepts](concepts.html) and shows how they relate to the `Fx.go` API, as well as providing a basic intuition on Effect requirements and evaluation.

## `Pure[V]`: Immediate Effects

The most basic of all possible effects are *Immediate* effects. These are effects of type `Fx[Nil, V]`, meaning that they have no ability requirements (`Nil`) and evaluate to a value (`V`).

Immediate effects are created using the `Pure(V) Fx[Nil, V]` function.

As you can see, `Pure(V)` takes an *existing* value `V`, that means that a pure effect just holds an already known value instead of trying to compute it.

The value given to `Pure` can be retrieved by using `Eval(Fx[Nil, V]) V`. Only effects that have no requirements (`Nil`) can be evaluated.

```go
import ( fx "github.com/vic/fx.go" )

func PureExample() {
    v := "Hello World"
    // Code annotated with types for clarity
    var effect fx.Fx[fx.Nil, string] = fx.Pure(v)
    var result string = fx.Eval(effect)
    assert(result == v)
}
```

## `Fx[S, V]`: Pending effects

An effect `Fx[S, V]` where `S != Nil` is a pending effect that needs `S` to be _provided_ for computing `V`.

The most basic pending computation is one you are already familiar with: *A function*.

In the following example, the function `LengthOfString(string) int` can be expressed as an Effect of type `Fx[string, int]`. Meaning that in order to have a value of `int` you need first to provide an `string` value:

```go
func LengthOfString(s string) int {
    return len(s)
}

func FuncExample() {
    // Code annotated with types for clarity
    var effect fx.Fx[string, int] = fx.Func(LengthOfString)
    var requirement string = "Hello World"
    var provided fx.Fx[fx.Nil, int] = fx.Provide(effect, requirement)
    var result int = fx.Eval(provided)
    assert(result == len(requirement))
}
```

From the code above:

- `Func(func (S) V)` produces a _pending_ effect of type `Fx[S, V]`.
- `Provide(Fx[S, V], S)` discharges the `S` requirement and returns `Fx[Nil, V]`.  
    Note that *no computation* is performed in this step. `Fx[Nil, V]` is still a description of a program, and `V` has not been computed yet, nor any side-effect has been performed.
- `Eval(Fx[Nil, V])` will actually perform the computation of `V`. Since all `non-Nil` requirements have already been provided, the computation can be run.


These two are the most basic effects in `Fx.go`. More interesting effects will be presented as we explore the topics of effect Rquests and Handlers.