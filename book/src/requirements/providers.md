# Providing Requirements

*Effect requirements can be provided in any order* with no impact in the evaluation of the program.


## `Provide*` Combinators.

These functions are used to eliminate requirements from effects. Only when `Nil` is the only remaining requirement, the effect can be evaluated.

The result of provide functions will always be another effect, meaning that no computation has been performed yet, nor any side-effect produced. The result is just another effect but with less requirements.

```go
// Discharges the single S requirement.
func Provide(Fx[S, V], S) Fx[Nil, V]


// Discharges the requirement of A by providing it.
func ProvideLeft(Fx[And[A, B], V], A) Fx[B, V]

// Discharges the requirement of B by providing it.
func ProvideRight(Fx[And[A, B], V], B) Fx[A, V]

// Discharges both A and B
func ProvideBoth(Fx[And[A, B], V], A, B) Fx[Nil, V]



// Provides A, the first part of the left And.
func ProvideFirstLeft(Fx[And[And[A, C], And[B, D]], V], A) Fx[And[C, And[B, D]], V]

// Provides B, the first part of the right And.
func ProvideFirstRight(Fx[And[And[A, C], And[B, D]], V], B) Fx[And[And[A, C], D], V]

// Provides A and B, the first part of both Ands.
func ProvideFirsts(Fx[And[And[A, C], And[B, D]], V], A, B) Fx[And[C, D], V]
```
