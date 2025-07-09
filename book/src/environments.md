# First-Class Environments and Context Structs

Contexts in fx-rs are just Rust structs, supporting named, nested, and multiple abilities.

## Example: Composing Contexts

```rust
use fx::Fx;

struct Logger;
struct HttpClient;
struct AppContext { logger: Logger, http: HttpClient }

fn log_and_fetch<'f>(ctx: &'f AppContext) -> Fx<'f, AppContext, ()> {
    Fx::pure(move |c: &AppContext| {
        // Use c.logger and c.http
    })
}
```

This makes dependency management explicit, readable, and maintainable.
