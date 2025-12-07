# Step-by-Step: Create Your Own `cargo hyp-myproject`

This guide shows how to set up a **standalone crate** (or workspace member) that produces
a custom `cargo` subcommand combining Hyp's built-in checkers with your own.

## Architecture Overview

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

## Creating your own hyp-myproject

### 1. Create the project

```bash
cargo new cargo-hyp-myproject --bin
cd cargo-hyp-myproject
```

### 2. Add dependencies (`Cargo.toml`)

```toml
[package]
name = "cargo-hyp-myproject"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "cargo-hyp-myproject"
path = "src/main.rs"

[dependencies]
hyp-analyzer = { git = "https://github.com/user/hyp", branch = "main" }
# or from crates.io once published:
# hyp-analyzer = "0.1"

syn = { version = "2", features = ["full", "visit"] }
clap = { version = "4", features = ["derive"] }
anyhow = "1"
```

### 3. Define your custom checkers

Create `src/checkers/mod.rs` and individual checker files. Example structure:

```
src/
├── main.rs
└── checkers/
    ├── mod.rs
    ├── e2001_no_user_generics.rs   # E2xxx: Custom Rust rules
    ├── e3001_dto_location.rs       # E3xxx: Repo layout rules
    └── e4001_transaction_leak.rs   # E4xxx: Business logic rules
```

Use the `define_checker!` macro (same as Hyp's built-in checkers):

```rust
// src/checkers/e4001_transaction_leak.rs
use hyp_analyzer::{define_checker, violation::Violation, checker::Checker};
use syn::visit::Visit;

define_checker! {
    /// E4001: Detects database queries outside transaction context
    E4001TransactionLeak,
    code = "E4001",
    name = "Database query outside transaction",
    suggestions = "Wrap database operations in a transaction block",
    target_items = [Function],
    config_entry_name = "e4001_transaction_leak",
    config = E4001Config {
        enabled: bool = true,
        severity: hyp_analyzer::config::SeverityLevel =
            hyp_analyzer::config::SeverityLevel::High,
        categories: Vec<hyp_analyzer::config::CheckerCategory> =
            vec![hyp_analyzer::config::CheckerCategory::Compliance],
    },
    check_item(self, item, file_path) {
        let mut visitor = TransactionVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

struct TransactionVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E4001TransactionLeak,
}

impl<'a> Visit<'a> for TransactionVisitor<'a> {
    fn visit_expr_method_call(&mut self, node: &'a syn::ExprMethodCall) {
        // Detect db.query(...) calls not inside transaction blocks
        let method_name = node.method.to_string();
        if method_name == "query" || method_name == "execute" {
            // Simplified: flag any direct query call
            use syn::spanned::Spanned;
            let span = node.span().start();
            self.violations.push(Violation::new(
                self.checker.code(),
                self.checker.name(),
                self.checker.severity().into(),
                "Database query called without explicit transaction context",
                self.file_path,
                span.line,
                span.column + 1,
            ).with_suggestion(self.checker.suggestions()));
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}
```

Export checkers in `src/checkers/mod.rs`:

```rust
pub mod e4001_transaction_leak;
pub use e4001_transaction_leak::{E4001TransactionLeak, E4001Config};

// Add more custom checkers here...
```

### 4. Create the CLI binary (`src/main.rs`)

```rust
use anyhow::Result;
use clap::Parser;
use hyp_analyzer::{
    cli_helper::{print_checker_list_from_registrations, run_cli, CliOptions, CliOutputFormat},
    registry::{get_all_checkers, CheckerRegistration},
    register_checker,
    checker::Checker,
    CheckerCategory,
};
use std::collections::HashSet;
use std::path::PathBuf;

mod checkers;
use checkers::{E4001TransactionLeak, E4001Config};

#[derive(Parser)]
#[command(name = "cargo-hyp-myproject")]
#[command(about = "Custom Hyp analyzer for MyProject", long_about = None)]
#[command(version)]
struct Cli {
    /// When invoked as `cargo hyp-myproject`, cargo passes "hyp-myproject" as first arg
    #[arg(hide = true)]
    _cargo_subcommand: Option<String>,

    #[arg(short, long, default_value = ".")]
    source: PathBuf,

    #[arg(short, long, default_value = "hyp.yaml")]
    config: PathBuf,

    #[arg(long)]
    all: bool,

    #[arg(long)]
    include: Option<String>,

    #[arg(long)]
    exclude: Option<String>,

    #[arg(long)]
    severity: Option<u8>,

    #[arg(long)]
    category: Option<String>,

    #[arg(short = 'f', long, default_value = "text")]
    format: String,

    #[arg(short = 'l', long)]
    list: bool,

    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,
}

/// Combine Hyp's built-in checkers with your custom ones
fn all_registrations() -> Vec<CheckerRegistration> {
    let mut regs = get_all_checkers();  // All E1xxx built-in checkers

    // Add your custom checkers (E2xxx, E3xxx, E4xxx, ...)
    regs.push(register_checker!(E4001TransactionLeak, E4001Config));
    // regs.push(register_checker!(E2001NoUserGenerics, E2001Config));
    // regs.push(register_checker!(E3001DtoLocation, E3001Config));

    regs
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let categories = cli.category.as_ref().and_then(|cat_str| {
        let set: HashSet<CheckerCategory> = cat_str
            .split(',')
            .filter_map(|s| CheckerCategory::parse_category(s.trim()))
            .collect();
        if set.is_empty() { None } else { Some(set) }
    });

    let opts = CliOptions {
        source: cli.source,
        config_path: cli.config,
        severity: cli.severity,
        categories,
        all: cli.all,
        include: cli.include.map(|s| s.split(',').map(|p| p.trim().to_string()).collect()),
        exclude: cli.exclude.map(|s| s.split(',').map(|p| p.trim().to_string()).collect()),
        format: if cli.format == "json" { CliOutputFormat::Json } else { CliOutputFormat::Text },
        verbose: cli.verbose,
    };

    if cli.list {
        print_checker_list_from_registrations(&all_registrations());
        return Ok(());
    }

    run_cli(opts, all_registrations)?;
    Ok(())
}
```

### 5. Build and install

```bash
cargo build --release
cargo install --path .
```

### 6. Use it!

```bash
# Run from your project directory
cargo hyp-myproject --source src/ -v

# List all checkers (built-in + custom)
cargo hyp-myproject --list

# Exclude certain built-in checkers you don't want
cargo hyp-myproject --source src/ --exclude e1106

# Run only your custom checkers
cargo hyp-myproject --source src/ --include e4001
```

## Alternatively: Add Custom Hyp to Your Existing Project

Instead of a separate repository, you can add a local `tools/hyp-myproject/` crate
inside your main project:

```
my-project/
├── Cargo.toml          # workspace
├── src/
│   └── lib.rs
└── tools/
    └── hyp-myproject/
        ├── Cargo.toml
        └── src/
            ├── main.rs
            └── checkers/
                └── ...
```

Add to workspace `Cargo.toml`:

```toml
[workspace]
members = [".", "tools/hyp-myproject"]
```

Then run:

```bash
cargo run -p cargo-hyp-myproject -- --source src/ -v
```

### Example Custom Checker Categories

| Code Range | Category | Example Checks |
|------------|----------|----------------|
| **E2xxx** | Custom Rust Rules | Prohibit user-defined generics, no direct thread spawning, no `unsafe` |
| **E3xxx** | Repo Layout Rules | DTOs only in `api/`, no business logic in `routes/`, models in `domain/` |
| **E4xxx** | Business Logic | Transaction leak detection, API auth middleware required, no raw SQLx |

### Disabling Built-in Checkers

If some Hyp built-in checkers don't fit your project, exclude them:

**Via CLI:**
```bash
cargo hyp-myproject --exclude e1106,e1002
```

**Via `hyp.yaml`:**
```yaml
checkers:
  e1002_direct_unwrap_expect:
    enabled: false
  e1106_long_function:
    enabled: false
```

**Or don't include them at all** in `all_registrations()`:
```rust
fn all_registrations() -> Vec<CheckerRegistration> {
    // Only include specific groups
    use hyp_analyzer::registry::{CheckerGroup, checkers_for_groups};
    let mut regs = checkers_for_groups(&[CheckerGroup::E10]); // Only unsafe checkers

    // Add custom
    regs.push(register_checker!(E4001TransactionLeak, E4001Config));
    regs
}
```

## Resources

- **Built-in checker examples**: See `hyp/crates/hyp-analyzer/src/checkers/e10/` and `e14/`
- **CLI reference implementation**: See `hyp/crates/hyp-analyzer-cli/src/main.rs`
- **Checker implementation guide**: See [crates/hyp-analyzer/README.md](crates/hyp-analyzer/README.md)
