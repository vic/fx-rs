# Structured Concurrency in fx-rs

fx-rs models concurrency primitives (spawning, channels, async tasks) as abilities. This makes concurrent operations explicit, composable, and testable.

## Composable Concurrency

You can add concurrency abilities to your context, and write effectful code that requests concurrency without knowing the underlying executor. Handlers can swap between real and mock concurrency backends for robust testing.

## Example

```rust
trait Spawn {
    fn spawn<Fut: Future<Output = ()> + Send + 'static>(&self, fut: Fut);
}

// Add Spawn to your context and use it in effectful code.
```

______________________________________________________________________

This approach enables structured concurrency, deterministic testing, and modular integration with Rust's async ecosystem.
