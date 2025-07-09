# Testing Effectful Code

fx-rs makes it easy to test effectful code by swapping in test handlers or mocks.

## Example: Mocking an Ability

```rust
use fx::Fx;

trait Http { fn get(&self, url: &str) -> String; }
struct MockHttp;
impl Http for MockHttp { fn get(&self, url: &str) -> String { format!("mocked: {}", url) } }

fn fetch<'f, H: Http>(url: &'f str) -> Fx<'f, H, String> {
    Fx::pure(move |h: &H| h.get(url))
}

let fx = fetch("/test");
let result = fx.run(&MockHttp);
assert_eq!(result, "mocked: /test");
```

You can also capture outputs, use property-based testing, and swap handlers for deterministic tests.
