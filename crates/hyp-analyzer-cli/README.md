# Hyp Analyzer CLI

The reference command-line interface for the Hyp Rust code analyzer.

## Usage

```bash
# Analyze source code
cargo run --bin hyp -- --source src/ -v

# List all available checkers
cargo run --bin hyp -- --list

# Filter by severity (1=Low, 2=Medium, 3=High)
cargo run --bin hyp -- --source src/ --severity 3

# Include/exclude specific checkers
cargo run --bin hyp -- --source src/ --include e1001,e1002
cargo run --bin hyp -- --source src/ --exclude e1106

# Output as JSON
cargo run --bin hyp -- --source src/ -f json
```

## As a Template for Custom CLIs

This CLI serves as a **reference implementation** for building your own project-specific
`cargo hyp-myproject` command. Key patterns:

1. **Use `get_all_checkers()`** to include all built-in Hyp checkers
2. **Add custom checkers** with `register_checker!`
3. **Call `run_cli()`** with your registrations

See `src/main.rs` for the full implementation and the
[main README](../../README.md#building-custom-validators) for a step-by-step guide
on building your own custom Hyp CLI.
