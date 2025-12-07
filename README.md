> Version: 0.1, early preview

# Hyp - Rust Code Quality Analyzer

Hyp - A static code analyzer that identifies compilable but problematic Rust code patterns that violate your project patterns or confuse developers and LLMs, helping teams to write clearer, safer, and more maintainable code with lower efforts on code review.

## Conceptual Model

At a high level Hyp does two things:

- **Parses Rust code into an AST** using the `syn` crate.
- **Runs a set of applicable checkers over that AST**, where each checker looks for one specific kind of problem.

### Problem Categories and Codes

All problems Hyp can detect are grouped into **categories**. The built‑in categories focus on
purely **Rust-specific** issues:

- **E10** – Unsafe code (panics, unwrap, unsafe blocks, FFI)
- **E11** – Code surface complexity (long functions, many parameters, deep nesting)
- **E12** – Code pattern complexity (complex generics, lifetimes, trait bounds)
- **E13** – Error handling patterns
- **E14** – Type safety (overflow, division/modulo by zero, unchecked indexing)
- **E15** – Concurrency
- **E16** – Memory safety
- **E17** – Performance
- **E18** – API design

Within each category, individual problems are identified by **checker codes**,
similar to PEP8 or Clippy lints:

- `E1001` – Direct call of `panic!`
- `E1002` – Direct call of `unwrap` / `expect`
- `E1106` – Long function (too many lines)
- `E1401` – Integer overflow/underflow
- `E1402` – Division by zero
- `E1403` – Modulo by zero
- …and many more listed in `crates/hyp-analyzer/README.md`.

These codes are what you **enable/disable** in configuration and on the CLI:

- In YAML (`hyp.yaml`), by checker key (e.g. `e1001_direct_panic`, `e1106_long_function`).
- On the CLI, via `--include e1001,e1106` or `--exclude e1402`.

Additionally, you can assign categories and adjust the severity level for any problem, then run validators on only a selected subset of issues.

### Extending Categories for Your Project

Project owners are encouraged to add their own **local categories** in addition
to the built‑in Rust families. For example:

- **E2xx – Project‑specific Rust rules**
  e.g. “prohibit user‑defined generics” or “no direct thread spawning” for certain crates or folders.
- **E3xx – Repository layout rules**
  e.g. “DTOs must live only in `api/`”, “no business logic in `routes/`”.
- **E4xx – Business logic and security rules**
  e.g. “database access must go through access‑controlled repositories”,
  “every admin API must check authorization middleware”, “no raw SQLx usage”.

These custom categories use the **same mechanism** as built‑in ones:

- Define checkers with `define_checker!` in your own crate or in a local `src/my_hyp/` module.
- Group them under your own code ranges (E2001, E3001, E4001, …).
- Register them with `register_checker!` and expose them via your own CLI
  (for example `cargo hyp-myproject`).

You can also **disable any built‑in Hyp checkers** that don’t match your project’s style,
and end up with a fully customized analyzer tuned to your own needs.

## How Hyp Differs from Other Rust Tools

Hyp fills a unique niche in the Rust tooling ecosystem. Here's how it compares:

| Tool | Focus | Approach | When to Use |
|------|-------|----------|-------------|
| **Hyp** | Cognitive complexity, LLM-friendliness, custom business rules | AST pattern matching, pluggable checkers | Code clarity, team standards, project-specific rules |
| **Clippy** | Idiomatic Rust, common mistakes | Compiler plugin, 700+ lints | General code quality, learning Rust idioms |
| **Miri** | Undefined behavior detection | Interpreter-based execution | Testing unsafe code, catching UB at runtime |
| **Kani** | Formal verification of properties | Model checking, SAT/SMT solving | Proving correctness of critical algorithms |
| **Prusti** | Verification via specifications | Viper verifier, annotations | Formal contracts, pre/post conditions |
| **MIRAI** | Abstract interpretation | Static analysis of MIR | Finding bugs without running code |

### Key Differentiators

**Pluggable & Extensible**: Unlike compiler-integrated tools, Hyp is designed for **custom business logic checks**. Add your own project-specific rules without forking or modifying the tool.

**LLM-Aware**: Hyp specifically targets patterns that confuse AI code assistants—helping teams using agentic AI code-generation tools write more maintainable code.

**Cognitive Metrics**: While Clippy checks for correctness, Hyp also measures *understandability*—cyclomatic complexity, nesting depth, and patterns that overload human working memory.

**Build Your Own Analyzer**: Create project-specific `cargo` commands with custom checks for your domain (see [Building Custom Validators](#building-custom-validators)).

## Built-in checks

### Problem Examples
The `problem-examples` crate contains **compilable but complicated or unsafe Rust code** designed to illustrate real-world Rust patterns that:
- Compile successfully but are difficult to review or maintain by human developers or LLMs
- May cause runtime errors or undefined behavior
- Violate best practices despite being technically valid

Each example includes:
- **Severity rating** (LOW/MED/HIGH) - Impact on code safety and maintainability
- **LLM confusion score** - How likely the pattern confuses AI code assistants
- **Clear description** - What the problem is and why it matters
- **Mitigation strategies** - How to detect and fix the issue

The default `hyp` static analyzer detects these problematic patterns in real codebases and delivers actionable reports, empowering developers to address issues before committing code.

## Building Custom Validators

Hyp is designed to be extended with **project-specific checks**. You can create a custom
`cargo` command (e.g., `cargo hyp-myproject`) that combines Hyp's built-in checkers with
your own domain-specific rules—all in a separate repository or a local crate inside your project.

### Architecture Overview

```
┌───────────────────────────────────────────────────────────────────────┐
│            Your Custom CLI Binary (cargo-hyp-myproject)               │
├───────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐ ┌─────────────┐ ┌───────────────┐ ┌───────────────┐ │
│  │ Hyp Built-in │ │ Your Custom │ │   Your Repo   │ │ Your Business │ │
│  │  Checkers    │ │ Rust Checks │ │ Layout Checks │ │ Logic Checks  │ │
│  │   (E1...)    │ │   (E2...)   │ │   (E3...)     │ │   (E4...)     │ │
│  └──────────────┘ └─────────────┘ └───────────────┘ └───────────────┘ │
├───────────────────────────────────────────────────────────────────────┤
│                 hyp-analyzer library with helpers                     │
└───────────────────────────────────────────────────────────────────────┘
```

See [crates/hyp-analyzer-cli/BUILD_YOUR_OWN_HYP_CLI.md](crates/hyp-analyzer-cli/BUILD_YOUR_OWN_HYP_CLI.md) for complete guide.

## Project Structure

This workspace contains 4 crates:

### Libraries

1. **problem-examples** - Compilable examples of problematic Rust code patterns
   - 100+ examples across 9 categories (E10-E18)
   - Each example demonstrates a specific anti-pattern
   - Intentionally disables clippy warnings to compile
   - Used for testing, documentation, and education

2. **hyp-analyzer** - The core static analysis library
   - AST-based pattern detection using `syn`
   - `define_checker!` and `register_checker!` macros for easy extension
   - Pluggable checker architecture
   - YAML-based configuration

### CLI Tools

3. **problem-examples-cli** - Interactive explorer for problem examples
   ```bash
   cargo run --bin problem-examples list
   cargo run --bin problem-examples show e10
   ```

4. **hyp-analyzer-cli** - Main analyzer CLI tool that can be used as is or as hyp-custom example
   ```bash
   cargo run --bin hyp -s src/ -v
   cargo run --bin hyp --list
   ```

### Example Problems Hyp Catches

```rust
// E1015: unwrap without context - confuses LLMs about error handling
let value = data.unwrap();  // ❌ What could go wrong here?

// E1101: high cyclomatic complexity - too many paths to reason about
fn process(x: i32, y: i32, z: i32) -> i32 {
    if x > 0 { if y > 0 { if z > 0 { /* ... */ } } }  // ❌ Hard to follow
}

// E1201: overly complex generics - LLMs struggle with type inference
fn complex<T, U, V, W, X>(_a: T, _b: U, _c: V, _d: W, _e: X)  // ❌ Too abstract
where T: Clone + Send + Sync + Debug + 'static, /* ... */
```

## Built-in Problem Categories

- **E10** - Unsafe Code (panics, unwrap, unsafe blocks, FFI)
- **E11** - Code Surface Complexity (cyclomatic complexity, long functions, many parameters)
- **E12** - Code Pattern Complexity (complex generics, lifetimes, trait bounds)
- **E13** - Error Handling (ignored results, poor error types, panic in Drop)
- **E14** - Type Safety (overflow, division by zero, unchecked indexing)
- **E15** - Concurrency (race conditions, deadlocks, non-Send types)
- **E16** - Memory Safety (use-after-free, dangling references, Rc cycles)
- **E17** - Performance (unnecessary allocations, inefficient data structures)
- **E18** - API Design (glob imports, public fields, poor naming)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/hyp.git
cd hyp

# Build all crates
cargo build

# List all available checkers
cargo run --bin hyp -- --list

# Run the analyzer on problem examples
cargo run --bin hyp -- -s crates/problem-examples/src -v

# Explore problem examples
cargo run --bin problem-examples list
cargo run --bin problem-examples show e10

# Run tests
cargo test
```

## Development Status

**Overall status**: proof-of-concept, actively developed
**Validators**: 27 checkers implemented across E10 (unsafe), E11 (complexity), E14 (type safety)
See detailed roadmap in [crates/hyp-analyzer/README.md](crates/hyp-analyzer/README.md)

## Target Audience

Hyp is designed for:

- **Rust learners** - Understand common pitfalls through compilable examples
- **Development teams** - Maintain consistent code quality and readability
- **AI-assisted development** - Write code that's clear to both humans and LLMs
- **Code reviewers** - Identify subtle issues that traditional linters miss
- **Library authors** - Ensure APIs are intuitive and hard to misuse
- **Platform teams** - Enforce project-specific conventions and business rules

## Contributing

Contributions are welcome! Areas where you can help:

1. **Add problem examples** - Found a confusing Rust pattern? Add it to `problem-examples/`
2. **Improve descriptions** - Make explanations clearer for learners
3. **Build analyzer rules** - Implement detection for existing problem categories
4. **Test and report** - Try Hyp on real codebases and report findings
5. **Documentation** - Improve guides, examples, and API docs

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Related Projects

- **[Clippy](https://github.com/rust-lang/rust-clippy)** - Comprehensive Rust linter (correctness and idioms)
- **[Kani](https://github.com/model-checking/kani)** - Bit-precise model checker for Rust
- **[Miri](https://github.com/rust-lang/miri)** - Interpreter for detecting undefined behavior
- **[Prusti](https://github.com/viperproject/prusti-dev)** - Verification via Viper
- **[MIRAI](https://github.com/facebookexperimental/MIRAI)** - Abstract interpreter for Rust

Hyp complements these tools by focusing on **cognitive complexity, LLM-friendliness, and custom code validation**.

## License

Apache-2.0
