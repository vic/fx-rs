# Concepts Tour

This section will try to introduce you to the concepts of
_Effects_, _Abilities_ and _Handlers_ as present in `Fx.go`.

No knowledge or previous experience with other effect sytems
is expected. We will try to explain things by
working out from simple concepts to more interesting ones.

## Effects

An *Effect* ( `Fx[S, V]` read: `V` provided `S` ) is the _description_ of a program that computes `V`, *provided* that the requirement `S` is present, so that the computation of `V` can be performed.

Since effects are *description*s of programs, they compute nothing nor produce side-effects until they are finally evaluated, once all their requirements are at met.

Some people also use the *recipe* analogy for effects: you first have a precise description of each step it takes to cook something, along with a list of the requirements for it (the ingredients and utencils you will use) and once you have them all, you can actually perform the recipe.

## Abilities

In `Fx[S, V]`, `S` is said to be the *Ability* (sometimes also referred as the _set_ of *Abilities*, *Capabilities*, *Effect Environment* or *Effect Requirements*) that are needed for computing `V`.

Abilities describe the external resources that would be needed, as well as the _side-effects_ that are possible while computing `V`.

Examples of such Abilities are:

- network abilities (eg, performing http requests)
- console abilities (eg, printing to the terminal or reading user input)
- non-deterministic abilities (eg, generating random numbers or coin-flips)
- resource handling (eg, disciplined acquire/use/release of shared/limited resources)
- exception handling (eg, interruption/resumption and finalizers)
- anything else that interacts with the world outside of the program.


## Handlers

A *Handler* for the `S` ability is a particular _interpretation_ of what `S` means.

Handlers are the only _side-effectful_ portion of your programs. It is possible, and quite common, to have different handlers (interpretations) for the same Ability, and each Handler  decides _how/when_ to perform world-modifying _side-effects_.

For example, for an _http-request_ ability you can have a *test-handler* that just mock responses to fixed values so that you can easily assert on known values on your tests. You could also have a *live-handler* that actually performs requests via the network for production runs.