# Future Work & Research Directions

fx-rs is designed to be extensible and to inspire further research and development in effect systems for Rust. Some promising directions and open questions include:

- **Modular Reasoning & Scoped Handlers:**

  - Advanced patterns for modular, component-based architectures using fx-rs, where each module declares its effect signature and dependencies as traits.
  - Scoped handler replacement (`Fx::adapt`) for targeted testing, feature flags, and environment-specific behavior.
  - Building up application context from smaller module contexts for scalable composition.

- **Effect Row Inference and Minimization:**

  - Static and dynamic effect row inference for scalable effect systems.
  - Linter and IDE integration for effect signature inference and minimization.
  - Exploring trade-offs between manual and inferred effect signatures.

- **Security and Capability Effects:**

  - Modeling permissions and capabilities as effects/abilities.
  - Compile-time enforcement of security policies via capabilities.
  - Patterns for capability injection, fine-grained permission checks, and compile-time security.

- **First-Class, Type-Safe Effect Scopes:**

  - Local reasoning and effect masking (scoping effects to regions).
  - Encoding effect scopes in Rust's type system.
  - Lexical vs. dynamic scoping for effects.

- **Resumable and Multi-Shot Handlers:**

  - Support for resumable exceptions, continuations, and multi-shot handlers.
  - Encoding resumable/multi-shot handlers in Rust and exploring their use cases.

If you are interested in contributing to any of these areas, or have ideas for new features, please open an issue or discussion on the fx-rs repository!
