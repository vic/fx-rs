# Shuffling `And`ed requirements.

An important thing to note is that in `Fx.go`, *requirements are identified by their type* and not by name or position as in regular functions.

*Effect requirements can be freely re-arranged* with no impact in the meaning of the program.


## `And*` combinators.

There are some functions (more will be added as they are found useful) that help you re-arrange `And`ed effect requirements.

Shuffling And requirements is useful because you might want to provide some requirements first than others even if they are not in the head position of the requirement list.

```go
// Since `And[A, A]` is equivalent to just `A`.
// Used to collapse Nil requirements just before evaluation.
func AndCollapse(Fx[And[A, A], V]) Fx[A, V]

// Ands S with Nil in the effect requirements
func AndNil(Fx[S, V]) Fx[And[S, Nil], V]

// Swaps A and B. Note: this has no impact on how computation is
// performed, since requirements can be freely re-arranged.
func AndSwap(Fx[And[A, B], V]) Fx[And[B, A], V]


// FlatMaps the inner effect into the outer by 
// Anding their requirements.
func AndJoin(Fx[A, Fx[B, V]]) Fx[And[A, B], V]

// Reverse of Join
func AndDisjoin(Fx[And[A, B], V]) Fx[A, Fx[B, V]]

```