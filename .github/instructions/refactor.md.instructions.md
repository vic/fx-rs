______________________________________________________________________

## applyTo: '\*\*'

Coding standards, domain knowledge, and preferences that AI should follow.

# AI Agent Integrity Pledge

- I, the AI coding agent, will NEVER lie or make unverified claims about test status, code correctness, or build results.
- I will ALWAYS verify test discovery and results using `cargo test`, `cargo test --list`, or equivalent commands, and will only assert test status based on direct evidence from these tools.
- I will never claim a test is enabled, passing, or failing unless I have checked it myself using the test system.
- I will strictly follow this pledge and all other instructions in this file for all future actions in this codebase.

# Instructions for AI Coding Agents

These are strict operational rules for AI coding agents working on this codebase. You must read and understand these instructions fully before proceeding with any coding tasks. You must confirm that you have read, understood, and will strictly follow these instructions.

## General Workflow

- **Test driven development.**
  Instead of adding comments, add TESTS!. When making a big
  change, add TESTS to make sure things will keep working.
  When making big changes, you can run only selected tests
  and progress to make ALL OF THEM WORK. Do this incrementally. NEVER make big changes at once without
  running tests.

- **Always run tests after any code modification.**\
  If any test fails, you must fix the code and rerun the tests until all pass. Never ask the user to run tests—always do it yourself.

- **Never say the code has errors UNLESS you have checked by using tests or compile**
  DO NOT assume code is correct NOR incorrect, PROVE IT.

- **Do not add comments to code unless absolutely critical for future AI edits.**\
  Comments are for AI maintenance only, not for human readers.\
  If an example is needed, add it as a test case in the appropriate module, not as a comment or doc-test.
  NEVER add comments that are FALSE. Prefer to not have comments than them being false assertions about code.

- **The `crates/fx` crate must remain macro-free and must not depend on any other internal crate.**\
  If macros are needed, implement them in a separate crate under `crates/`.

- **System-wide or integration tests must go in the `crates/integration` crate, which can depend on all other crates.**

- **Make strategic, incremental changes:**

  - Tackle base problems first, then add features.
  - If an API or architectural decision is needed, ask the user; otherwise, proceed autonomously.

- **Use `jj` (jujutsu) for version control. Never use git.**

  - Always use `--no-pager` so you are able to see jj output.
  - Use `jj new` to create a new changeset when you have a stable point.
  - Use `jj describe -m "message"` to describe the current changeset, using semantic commit messages (short summary, then details). NEVER include statements as "all tests pass", "no warings" etc. Message should talk about design decisions, and impl details. These messages are intended to be read by YOU in the future.
  - Use `jj diff --no-pager` to confirm your changes before describing or advancing.
  - After each important, tested step, create a new change and describe it.
  - You may use other `jj` commands as needed (`jj show`, `jj log`, `jj branch`, etc.).

- **Avoid Rust nightly unless strictly necessary.**

- **Never use 'static nor Any.**

- **Never use internal mutability** (`RefCell`, `Arc`, or mutable references) unless absolutely required. If you believe it is necessary, ask the user before proceeding.

- **Always use FnOnce. You are allowed to use FnOnce+Clone when needed. But never call .clone() explicitly unless absolutely needed.**

- **Only use `.clone()` when truly needed, following the code style.**

- **Always typecheck and verify function signatures and return types.**

- **Use tests to verify all changes.**

- **Never leave warnings in the code.**\
  If there are warnings, fix or remove the code that produces them.

- **Do not use doc-tests for examples.**\
  All examples must be implemented as proper test cases.

- **Format code after successful tests And before describing the current changeset using `nix fmt`.**

- **When searching the codebase, use the `rg` (ripgrep) tool from the command line.**

- **When you need to fetch or inspect web content, use `curl` from the command line.**

- **If the user says only `jj`, you must:**

  1. Use `jj describe -m "your message"` with a semantic commit message to describe the current change.
  1. Use `jj new` to create a new changeset.

- **Approach problems in steps:**

  - Start with simple, test-driven or incremental strategies.
  - Refactor in small, verifiable steps.

- **You must confirm you have read, understood, and will strictly follow these instructions before starting any coding work.**

**Make a summary of this instructions SO THAT I KNOW YOU HAVE UNDERSTOOD THEM**.

______________________________________________________________________

We are in the middle of a refactor. Our strategy is
this:

- **Keep all code that does not compile COMMENTED**
- **Always use single line comments using two slashes `//`**
- **We will try to pick a test that thas THE LESS commented-dependencies**
- **We will enable this SINGLE TEST by moving it to the tests/ directory on the crate.**
- **We will make sure the test is enabled by using `cargo test -- --list`**
- **If integration tests seem not to appear, use cargo clean or check or compile**
- **We will FOCUS on making this single test PASS**
- **When the test is OK we iterate again.**

______________________________________________________________________

AI *MUST* aknowledge its understanding by pledging a summary:
