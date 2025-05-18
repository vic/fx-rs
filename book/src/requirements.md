# Effect Requirements

So far, we have seen that an effect `Fx[S, V]` can have `S` be `Nil` for effects that can be evaluated right away and non-`Nil` for those pending effects that still need to be provided some value.

In this chapter we will talk about aggregating requirements using the `And` type. And how different types of functions/effects can represent the very same computation. We also look at similarities with higher-order functions in functional-programming and how rotating or re-arranging effect requirements is indifferent in `Fx.go`. Finally, we show some the`And*` and `Provide*` combinators that can help you reshape your effect requirements.

## Composite requirements

Using the same "length of string" function from the previous chapters, we can describe it in different ways. 

```go
// This is an strange way of writing `func(string) int`.
// But this shape can help to understand the types bellow.
//
// Focus your attention on the requirements that are
// needed for a computing a value.
//
// In particular, note that `Fx[Nil, V]` is like a `func() V`
func LengthOfString(s string) func() int {
    return func() int { return len(s) }
}

// The type of LengthOfString expressed as an effect request.
type LenFn = func(string) Fx[Nil, int]
```

Note that all of the following types are equivalent, as they describe the very same requirements and result types:
- `func(string) int`
- `Fx[string, int]`
- `func(string) func() int`
- `func(string) Fx[Nil, int]`
- `Fx[string, Fx[Nil, int]]`
- `Fx[And[string, Nil], int]`
- `Fx[And[Nil, string], int]`

The last three examples represent nested effects and are equivalent to functions of arity > 1 or functions that return functions.

`And[A, B]` is the requirement for both `A` and `B` abilities. Notice in the last two examples, that their components are swapped. It is important to note that in `Fx.go`, _the *order* of the abilities on And requirements does not matter_ and they can be freely swapped/joined/unjoined. More on this when we talk about `And*` combinators.

Also, note that `And[A, Nil]` is equivalent to just `A`. All of these types represent the same type of computation and an effect can be transformed to any of those types freely.



## `>1` arity functions as effects.

Suppose you have a function that multiplies an string length by n.

```go
func MulLen(s string, n int) int {
    return len(s) * n
}
```

`MulLen` can be described by the following types:

- `func(string, int) int`
- `func(string) func(int) int`
- `Fx[And[string, int], int]`
- `Fx[string, Fx[int, int]]`
- `Fx[int, Fx[string, int]]`
- `Fx[And[int, string], int]`

Note that `And[X, X]` is equivalent to just a single `X` requirement, and that `And[And[X, Y], And[Y, X]]` is also equivalent to `And[X, Y]`.


