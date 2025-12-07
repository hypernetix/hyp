//! Hyp Analyzer CLI
//!
//! Command-line interface for the Hyp Rust code analyzer.

use anyhow::Result;
use clap::Parser;
use hyp_analyzer::{
    cli_helper::{
        print_checker_list_from_registrations, print_guidelines_from_registrations, run_cli,
        CliOptions, CliOutputFormat,
    },
    registry::CheckerRegistration,
    CheckerCategory,
};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hyp-analyzer")]
#[command(about = "Hyp Rust Code Analyzer - Detect code problems and anti-patterns", long_about = None)]
#[command(version)]
struct Cli {
    /// Path to source code (file or directory, defaults to current directory)
    #[arg(short, long, default_value = ".")]
    source: PathBuf,

    /// Path to configuration file (YAML)
    #[arg(short, long, default_value = "hyp.yaml")]
    config: PathBuf,

    /// Enable all checkers (overrides config)
    #[arg(long)]
    all: bool,

    /// Include only specific checkers (comma-separated, supports substring matching).
    /// Examples: "e1001" (exact), "e10" (e1000-e1099), "e1" (e1001-e1999)
    #[arg(long)]
    include: Option<String>,

    /// Exclude specific checkers (comma-separated, supports substring matching).
    /// Examples: "e1201" (exact), "e12" (e1200-e1299), "e1" (e1001-e1999)
    #[arg(long)]
    exclude: Option<String>,

    /// Minimum severity level (1, 2, or 3). Only checkers with this severity or higher will run.
    /// 1=Low (style), 2=Medium (potential issues), 3=High (critical/unsafe)
    #[arg(long)]
    severity: Option<u8>,

    /// Filter by categories (comma-separated: operations, complexity, compliance)
    /// Only checkers in these categories will run
    #[arg(long)]
    category: Option<String>,

    /// Output format (text or json)
    #[arg(short = 'f', long, default_value = "text")]
    format: String,

    /// List all available checkers with their severity and categories
    #[arg(short = 'l', long)]
    list: bool,

    /// Print condensed guidelines for LLMs based on enabled checkers
    #[arg(short = 'g', long)]
    guideline: bool,

    /// Verbose mode: -v for info, -vv for debug
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,
}

/// Default checker registrations used by this CLI binary.
///
/// This CLI supports two registration modes:
///
/// ## Mode 1: Group-based registration (DEFAULT, recommended)
/// Register entire checker groups (e10, e11, etc.) at once.
/// This is the simplest approach and automatically includes all checkers
/// in the selected groups.
///
/// Example:
/// ```rust
/// use hyp_analyzer::registry::{CheckerGroup, checkers_for_groups};
///
/// fn default_registrations() -> Vec<CheckerRegistration> {
///     // Register all E10 (unsafe/panic) and E11 (complexity) checkers
///     checkers_for_groups(&[CheckerGroup::E10, CheckerGroup::E11])
/// }
/// ```
///
/// ## Mode 2: Individual checker registration
/// Explicitly register each checker. Use this when you need fine-grained
/// control or want to mix checkers from different groups with custom ones.
///
/// Example:
/// ```rust
/// fn default_registrations() -> Vec<CheckerRegistration> {
///     vec![
///         CheckerRegistration {
///             descriptor: E1001DirectPanic::descriptor(),
///             factory: |config| {
///                 let cfg = config.e1001();
///                 if cfg.enabled {
///                     Some(Box::new(E1001DirectPanic::new(cfg)))
///                 } else {
///                     None
///                 }
///             },
///         },
///         // ... more individual checkers
///     ]
/// }
/// ```
///
/// You can also mix both approaches:
/// ```rust
/// fn default_registrations() -> Vec<CheckerRegistration> {
///     let mut regs = checkers_for_groups(&[CheckerGroup::E10]);
///
///     // Add a custom checker
///     regs.push(CheckerRegistration {
///         descriptor: MyCustomChecker::descriptor(),
///         factory: |config| { /* ... */ },
///     });
///
///     regs
/// }
/// ```
fn default_registrations() -> Vec<CheckerRegistration> {
    // MODE 1 (DEFAULT): Use all available checkers from the registry
    use hyp_analyzer::registry::get_all_checkers;
    get_all_checkers()

    // MODE 2 (ALTERNATIVE): Individual checker registration
    // Uncomment this and comment out the above if you want explicit control:
    /*
    let e1001_desc = E1001DirectPanic::descriptor();
    let e1106_desc = E1106LongFunction::descriptor();

    vec![
        CheckerRegistration {
            descriptor: e1001_desc,
            factory: |config| {
                let cfg = config.e1001();
                if cfg.enabled {
                    Some(Box::new(E1001DirectPanic::new(cfg)))
                } else {
                    None
                }
            },
        },
        CheckerRegistration {
            descriptor: e1106_desc,
            factory: |config| {
                let cfg = config.e1106();
                if cfg.enabled {
                    Some(Box::new(E1106LongFunction::new(cfg)))
                } else {
                    None
                }
            },
        },
    ]
    */
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Map CLI flags into generic CliOptions
    let categories = if let Some(cat_str) = &cli.category {
        let set: HashSet<CheckerCategory> = cat_str
            .split(',')
            .filter_map(|s| {
                let trimmed_s = s.trim();
                if let Some(category) = CheckerCategory::parse_category(trimmed_s) {
                    Some(category)
                } else {
                    eprintln!("Warning: Unknown category '{}', ignoring", trimmed_s);
                    eprintln!("Valid categories: operations, complexity, compliance");
                    None
                }
            })
            .collect();
        if set.is_empty() {
            None
        } else {
            Some(set)
        }
    } else {
        None
    };

    let opts = CliOptions {
        source: cli.source.clone(),
        config_path: cli.config.clone(),
        severity: cli.severity,
        categories,
        all: cli.all,
        include: cli.include.as_ref().map(|s| split_csv(s)),
        exclude: cli.exclude.as_ref().map(|s| split_csv(s)),
        format: match cli.format.as_str() {
            "json" => CliOutputFormat::Json,
            _ => CliOutputFormat::Text,
        },
        verbose: cli.verbose,
    };

    // --list handled here; must respect config + CLI filters
    if cli.list {
        print_checker_list_from_registrations(&opts, default_registrations())?;
        return Ok(());
    }

    // --guideline handled here; must respect config + CLI filters
    if cli.guideline {
        print_guidelines_from_registrations(&opts, default_registrations())?;
        return Ok(());
    }

    // For this CLI we use explicit registrations; another CLI could pass groups instead.
    run_cli(opts, default_registrations)?;
    Ok(())
}

fn split_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}
