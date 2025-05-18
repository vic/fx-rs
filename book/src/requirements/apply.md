# Functional Requirements

As we have seen in the previous [Context](context.html) chapter, you can ask for any type to be part of an effect's environment. 

You can also ask for functions of some type to be present and use them in your program descriptions without knowing their exact implementation.

For example, suppose we ask for a function that takes an integer. And then apply it to a value.

```go
// The type of the function from int to int
type OnInt func(int) nit

// Apply OnInt(12)
var a Fx[OnInt, int] = Map(Ctx[OnInt](), func(f OnInt) int {
    return f(12)
})

// fx.Apply is just an abbreviation of the previous code:
// It takes the type of a unary function and the value to apply.
var b Fx[OnInt, int] = Apply[OnInt](12)
```

Because `OnInt` is part of the environment, we only know its signature (its request and response type), but not the actual implementation of it. This way, different implementations of `OnInt` can be provided later.


Now, suppose you have the following code:

```go
type HttpRq string
var url = HttpRq("https://example.org")

// `Http` is the type of an *effect request* function.
// it takes a plain value and creates an effect.
type Http func(HttpRq) Fx[HttpService, HttpRs]

// NOTE: applying to Http, creates a nested effect
var a Fx[Http, Fx[HttpService, HttpRs]] = Apply[Http](url)

// Flatenning creates an Anded environment.
var b Fx[And[Http, HttpService], HttpRs] = AndJoin(a)


// Same as previous two lines:
var x Fx[And[Http, HttpService], HttpRs] = Suspend[Http](url)
```

`fx.Suspend` is an abbreviation of `AndJoin(Apply[Http](url))` for *effect requests* (functions returning an effect)


This is the principle behind *suspended effect application* in `Fx.go` and is a fundamental part when we talk about effect [Requests](../requests.html) and [Handlers](../handlers.html).

