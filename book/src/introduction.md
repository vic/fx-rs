# Introduction

An Algebraic Effects System for Rust.

[![fx-go](https://github-readme-stats.vercel.app/api/pin/?username=vic&repo=fx-rs&show_owner=true&theme=shades-of-purple)](https://github.com/vic/fx-rs)

<div class="warning">
Fx.rs is currently experimental.

API surface is *very much* in flux and evolving.

New effects will be added as they are discovered to be useful in the Rust ecosystem.

</div>

### How are Algebraic Effects useful?

Algebraic Effects are useful because they allow programs to
be expressed not only in terms of what kind of value they can
compute but also on what possible side-effects or external resources will such a computation require.

By using Effect Handlers, the interpretation of how an effect is performed is independent of the program description. This means that a single program description can be interpreted in different ways. For example, using a *test-handler* that mocks request to external services, or using a *live-handler* that actually performs such requests.

If you want to read more about different language implementations and theory behind effects, read the [effects-bibliography](https://github.com/yallop/effects-bibliography).

`Fx.rs` is inspired by the following two implementations, and uses a similar notion of the _Handler_, _Ability_, and _Effect_ concepts:

- [Unison Abilities](https://www.unison-lang.org/docs/language-reference/abilities-and-ability-handlers/)
- [Kyo (Scala3)](https://github.com/getkyo/kyo/) - special thanks to [@fbrasisil](https://x.com/fbrasisil), Kyo's author who kindly provided a minimal kyo-core that helped [me](https://x.com/oeiuwq) understand algebraic effect systems and inspired this library.
- [Fx.go] my original library for Golang.

However, `Fx.rs` has a different surface API since we are trying to provide the best dev-experience for Rust programmers.
