# Dependency Injection and Programmable Typeclasses

fx-rs unifies algebraic effects and dependency injection. Handlers are first-class values, not static typeclasses, so you can swap them at runtime and scope them to subcomputations.

## Example: Swapping Handlers for a Subcomputation

```rust
use fx::Fx;

trait Logger { fn log(&self, msg: &str); }
struct ProdLogger;
impl Logger for ProdLogger { fn log(&self, msg: &str) { println!("prod: {}", msg); } }
struct TestLogger;
impl Logger for TestLogger { fn log(&self, msg: &str) { println!("test: {}", msg); } }

struct AppContext<L: Logger> { logger: L }

fn log_something<'f, L: Logger>(msg: &'f str) -> Fx<'f, L, ()> {
    Fx::pure(move |l: &L| l.log(msg))
}

let prod_ctx = AppContext { logger: ProdLogger };
let fx = log_something("main");
let _ = fx.run(&prod_ctx.logger);

// Swap to a test logger for a subcomputation
let test_ctx = AppContext { logger: TestLogger };
let _ = fx.adapt(|_| &test_ctx.logger, |c, r| (c, r)).run(&prod_ctx.logger);
```

This enables modular, testable, and flexible dependency management—without global singletons or static typeclasses.
