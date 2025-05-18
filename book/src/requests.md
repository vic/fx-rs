# Effect Requests

Another way of creating effects in `Fx.go` is via an *effect-request* function.

A function of type `func(I) Fx[S, O]` is said to take an _effect-request_ `I` and produce an *suspended* effect `Fx[S, O]`.

An example, is the function `func(HttpReq) Fx[HttpService, HttpRes]` we saw in the [previous](./requirements/apply.html) chapter.

Using the "length of string" example of the previous chapters, we can use it to model an effect request:

```go
type LenFn = func(string) fx.Fx[fx.Nil, int]

// Code is type annotated for clarity
var lenFx fx.Fx[fx.And[LenFn, fx.Nil], int] = fx.Suspend[LenFn]("hello")
```

Note that `Suspend` takes the _type_ of a request-effect function and the request value for it. And yields a *suspended* effect of type `Fx[And[LenFn, Nil], int]`. The computation is said to be *suspended* because it knows not what particular implementation of `LenFn` should be used, and because of this, `LenFn` is part of the requirements, along with `Nil` the ability requirement on the result of `LenFn`.

Different implementations of `LenFn` can be provided to the `lenFx` effect.

```go
var lies LenFn = func(_ string) fx.Fx[fx.Nil, int] {
    return fx.Pure(42)
}
var truth LenFn = func(s string) fx.Fx[fx.Nil, int] {
    return fx.Pure(len(s))
}

var x int = fx.Eval(fx.ProvideLeft(lenFx, lies))
assert(x == 42)

var y int = fx.Eval(fx.ProvideLeft(lenFx, truth))
assert(y == 5)
```

Notice that by delaying which implementation of `LenFn` is used, the `lenFx` program description includes the effect request `"hello"` and knows the general form of its response `Fx[Nil, int]`, but knows nothing about which particular interpretation of `LenFn` will be used.