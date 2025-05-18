# Handlers

A _Handler_ is an effect transformation function of type `func(Fx[R, U]) Fx[S, V]`.

Handlers are free to change the effect requirements, they tipically reduce the requirement set, but they could also introduce new requirements. They can also change or keep the result type.

## Handling an effect

Lets re-write our previous "length of string" function as a Handler.

```go
type LenFn func(string) Fx[Nil, int]

// Code is type annotated for clarity
var lenFx Fx[And[LenFn, Nil], int] = fx.Suspend[LenFn]("hello")

// type of a handler. not needed but added for clarity.
type LenHn func(Fx[And[LenFn, Nil], int]) Fx[Nil, int]

var handler LenHn = fx.Handler(func(s string) fx.Fx[fx.Nil, int] {
    return fx.Pure(len(s))
})

// apply the handler directly to lenFx
var x *int = fx.Eval(handler(lenFx))
assert(*x == 5)
```

As you might guess, `fx.Handler` is just a wrapper for `ProvideLeft(Fx[And[Fn, S], O], Fn) Fx[S, O]` where `Fn = func(I) Fx[S, O]`, an request-effect function.


## Requesting Handlers (effect-transformers) from the environment.

Of course, you can also request that a particular effect transformer (Handler) be present as a requirement of some computation. This way the handler is provided only once but can be applied anywhere it is needed inside the program.

```go
// Same examples as above with some more types for clarity

// effect-request function type.
type LenFn func(string) Fx[Nil, int]
// effect handler type
type LenHn = func(Fx[And[LenFn, Nil], int]) Fx[Nil, int]

// effect ability
type LenAb = And[LenHn, Nil]
// effect type producing V
type LenFx[V any] = fx.Fx[LenAb, V]

// Same as: Suspend[LenHn](Suspend[LenFn](input))
var lenFx LenFx = fx.Handle[LenHn]("hello")

var handler LenHn = fx.Handler(func(s string) fx.Fx[fx.Nil, int] {
    return fx.Pure(len(s))
})

// Now instead of applying the handler directly to each effect
// we provide it into the environment.
var provided Fx[Nil, int] = fx.ProvideLeft(lenFx, handler)
val x int = fx.Eval(provided)
assert(x == 5)
```