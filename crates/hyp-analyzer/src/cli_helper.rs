//! Shared helper functions for CLI frontends.

use std::{collections::HashSet, path::PathBuf};

use crate::{
    analyzer::{Analyzer, AnalyzerFilters},
    config::{AnalyzerConfig, CheckerCategory},
    registry::{checkers_for_groups, CheckerGroup, CheckerRegistration},
    violation::Violation,
    Result,
};

/// CLI output format options.
#[derive(Debug, Clone, Copy)]
pub enum CliOutputFormat {
    /// Plain text output.
    Text,
    /// JSON output.
    Json,
}

/// Generic CLI options structure that any CLI frontend can use.
pub struct CliOptions {
    /// Path to source code (file or directory).
    pub source: PathBuf,
    /// Path to configuration file.
    pub config_path: PathBuf,
    /// Minimum severity filter (1-3).
    pub severity: Option<u8>,
    /// Category filters.
    pub categories: Option<HashSet<CheckerCategory>>,
    /// Enable all checkers flag.
    pub all: bool,
    /// Include only specific checkers.
    pub include: Option<Vec<String>>,
    /// Exclude specific checkers.
    pub exclude: Option<Vec<String>>,
    /// Output format.
    pub format: CliOutputFormat,
    /// Verbosity level (0=normal, 1=info, 2=debug).
    pub verbose: u8,
}

/// Build an analyzer from an explicit list of checker registrations.
pub fn build_analyzer_from_registrations(
    config: AnalyzerConfig,
    filters: AnalyzerFilters,
    registrations: Vec<CheckerRegistration>,
) -> Analyzer {
    Analyzer::new_with_checkers(config, filters, registrations)
}

/// Build an analyzer from one or more logical checker groups.
pub fn build_analyzer_from_groups(
    config: AnalyzerConfig,
    filters: AnalyzerFilters,
    groups: &[CheckerGroup],
) -> Analyzer {
    let registrations = checkers_for_groups(groups);
    Analyzer::new_with_checkers(config, filters, registrations)
}

/// Apply CLI include/exclude filtering to registrations (case-insensitive).
///
/// Supports substring matching:
/// - "e1" matches e1001-e1999
/// - "e10" matches e1000-e1099
/// - "e1001" matches exactly e1001
pub fn filter_registrations(
    registrations: Vec<CheckerRegistration>,
    opts: &CliOptions,
) -> Vec<CheckerRegistration> {
    let include_patterns: Option<Vec<String>> = opts
        .include
        .as_ref()
        .map(|v| v.iter().map(|s| s.to_lowercase()).collect());
    let exclude_patterns: Option<Vec<String>> = opts
        .exclude
        .as_ref()
        .map(|v| v.iter().map(|s| s.to_lowercase()).collect());

    registrations
        .into_iter()
        .filter(|reg| {
            let code_lc = reg.descriptor.code.to_lowercase();

            // If include is present, only allow codes that match any include pattern
            if let Some(ref patterns) = include_patterns {
                let matches = patterns.iter().any(|pattern| code_lc.starts_with(pattern));
                if !matches {
                    return false;
                }
            }

            // Exclude overrides include - if code matches any exclude pattern, filter it out
            if let Some(ref patterns) = exclude_patterns {
                let matches = patterns.iter().any(|pattern| code_lc.starts_with(pattern));
                if matches {
                    return false;
                }
            }

            true
        })
        .collect()
}

/// Print a table of *eligible* checkers based on config + CLI filters.
pub fn print_checker_list_from_registrations(
    opts: &CliOptions,
    registrations: Vec<CheckerRegistration>,
) -> Result<()> {
    // Load config (same as run_cli)
    let config = if opts.config_path.exists() {
        let yaml = std::fs::read_to_string(&opts.config_path)?;
        AnalyzerConfig::from_yaml(&yaml)?
    } else {
        AnalyzerConfig::default()
    };

    // Build filters (severity/category already populated in opts)
    let mut filters = AnalyzerFilters::default();
    if let Some(sev) = opts.severity {
        filters.min_severity = Some(sev);
    }
    if let Some(cats) = opts.categories.clone() {
        filters.categories = Some(cats);
    }

    let registrations = filter_registrations(registrations, opts);
    let analyzer = build_analyzer_from_registrations(config, filters, registrations);

    println!("\nEligible Checkers:\n");
    println!("{:<8} {:<30} {:<10} Categories", "Code", "Name", "Severity");
    println!("{}", "-".repeat(80));

    for c in analyzer.enabled_checkers() {
        let severity_str = match c.severity {
            1 => "1 (Low)",
            2 => "2 (Medium)",
            3 => "3 (High)",
            _ => "Unknown",
        };
        println!(
            "{:<8} {:<30} {:<10} {}",
            c.code,
            c.name,
            severity_str,
            c.categories.join(", ")
        );
    }

    println!("\nTotal: {} checkers\n", analyzer.checker_count());
    Ok(())
}

/// Print condensed guidelines for LLMs based on enabled checkers.
pub fn print_guidelines_from_registrations(
    opts: &CliOptions,
    registrations: Vec<CheckerRegistration>,
) -> Result<()> {
    // Load config (same as run_cli)
    let config = if opts.config_path.exists() {
        let yaml = std::fs::read_to_string(&opts.config_path)?;
        AnalyzerConfig::from_yaml(&yaml)?
    } else {
        AnalyzerConfig::default()
    };

    // Build filters (severity/category already populated in opts)
    let mut filters = AnalyzerFilters::default();
    if let Some(sev) = opts.severity {
        filters.min_severity = Some(sev);
    }
    if let Some(cats) = opts.categories.clone() {
        filters.categories = Some(cats);
    }

    let registrations = filter_registrations(registrations, opts);
    let analyzer = build_analyzer_from_registrations(config, filters, registrations);

    println!("Do not use the following patterns:\n");

    // Get guidelines from the analyzer's enabled checkers
    for guideline in analyzer.enabled_guidelines() {
        println!("- {} - {} - {}", guideline.code, guideline.name, guideline.suggestions);
    }

    println!("\nTotal: {} guidelines", analyzer.checker_count());
    Ok(())
}

/// Main CLI runner that handles the full analysis workflow.
pub fn run_cli<F>(opts: CliOptions, make_registrations: F) -> Result<()>
where
    F: FnOnce() -> Vec<CheckerRegistration>,
{
    // 1. Load configuration
    let config = if opts.config_path.exists() {
        let yaml = std::fs::read_to_string(&opts.config_path)?;
        AnalyzerConfig::from_yaml(&yaml)?
    } else {
        AnalyzerConfig::default()
    };

    // 2. Build filters
    let mut filters = AnalyzerFilters::default();
    if let Some(sev) = opts.severity {
        filters.min_severity = Some(sev);
    }
    if let Some(ref cats) = opts.categories {
        filters.categories = Some(cats.clone());
    }

    // 3. Get registrations, apply include/exclude, and build analyzer
    let registrations = filter_registrations(make_registrations(), &opts);
    let analyzer = build_analyzer_from_registrations(config, filters, registrations);

    // 4. Print enabled checkers (non-verbose mode)
    if opts.verbose == 0 {
        eprintln!("Analyzing: {}", opts.source.display());
        eprintln!("\nEnabled checkers ({}):", analyzer.checker_count());
        for checker in analyzer.enabled_checkers() {
            let severity_str = match checker.severity {
                1 => "Low",
                2 => "Med",
                3 => "High",
                _ => "?",
            };
            eprintln!(
                "  - {} - {} [{}] ({})",
                checker.code,
                checker.name,
                severity_str,
                checker.categories.join(", "),
            );
        }
        eprintln!();
    }

    // 5. Run analysis (verbose vs normal)
    let violations = if opts.verbose > 0 {
        analyze_with_verbose(&analyzer, &opts.source, opts.verbose)?
    } else {
        analyzer.analyze(&opts.source)?
    };

    // 6. Sort violations by file path, then by code
    let mut violations = violations;
    violations.sort_by(|a, b| {
        a.file_path
            .cmp(&b.file_path)
            .then_with(|| a.code.cmp(&b.code))
    });

    // 7. Output formatting
    match opts.format {
        CliOutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&violations)?);
        }
        CliOutputFormat::Text => {
            if violations.is_empty() {
                println!("\nGood, No violations found!");
            } else {
                println!("\nFound {} violation(s):\n", violations.len());
                for v in &violations {
                    println!(
                        "[{}] {} - {}",
                        v.code,
                        v.name,
                        match v.severity {
                            crate::violation::Severity::High => "HIGH",
                            crate::violation::Severity::Medium => "MEDIUM",
                            crate::violation::Severity::Low => "LOW",
                        }
                    );
                    println!("  File: {}:{}", v.file_path, v.line);
                    println!("  {}", v.message);
                    if let Some(suggestion) = &v.suggestion {
                        println!("  Suggestion: {}", suggestion);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

/// Analyze with verbose output showing detailed progress.
pub fn analyze_with_verbose(
    analyzer: &Analyzer,
    path: &PathBuf,
    verbose_level: u8,
) -> Result<Vec<Violation>> {
    use walkdir::WalkDir;

    let mut all_violations = Vec::new();
    let mut files_analyzed = 0;
    let mut total_items = 0;

    eprintln!("\nVerbose Analysis (level: {})\n", verbose_level);

    eprintln!("Enabled checkers ({}):", analyzer.checker_count());
    for checker in analyzer.enabled_checkers() {
        let severity_str = match checker.severity {
            1 => "Low",
            2 => "Med",
            3 => "High",
            _ => "?",
        };
        eprintln!(
            "  - {} - {} [{}] ({})",
            checker.code,
            checker.name,
            severity_str,
            checker.categories.join(", ")
        );
    }
    eprintln!();

    let paths: Vec<_> = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().and_then(|s| s.to_str()) == Some("rs")
                    && !e.path().components().any(|c| c.as_os_str() == "target")
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    for file_path in paths {
        files_analyzed += 1;

        // Info mode (-v): print file being analyzed
        if verbose_level >= 1 {
            eprintln!("Analyzing: {}", file_path.display());
        }

        // Debug mode (-vv): parse and show AST items
        if verbose_level >= 2 {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                if let Ok(syntax) = syn::parse_file(&content) {
                    let items = &syntax.items;
                    total_items += items.len();

                    eprintln!("   Found {} items:", items.len());
                    for item in items {
                        let item_desc = match item {
                            syn::Item::Fn(f) => format!("   - fn {}", f.sig.ident),
                            syn::Item::Struct(s) => format!("   - struct {}", s.ident),
                            syn::Item::Enum(e) => format!("   - enum {}", e.ident),
                            syn::Item::Trait(t) => format!("   - trait {}", t.ident),
                            syn::Item::Impl(i) => {
                                if let Some((_, path, _)) = &i.trait_ {
                                    format!("   - impl {} for ...", quote::quote!(#path))
                                } else if let syn::Type::Path(p) = &*i.self_ty {
                                    format!("   - impl {}", quote::quote!(#p))
                                } else {
                                    "   - impl ...".to_string()
                                }
                            }
                            syn::Item::Mod(m) => format!("   - mod {}", m.ident),
                            syn::Item::Const(c) => format!("   - const {}", c.ident),
                            syn::Item::Static(s) => format!("   - static {}", s.ident),
                            syn::Item::Type(t) => format!("   - type {}", t.ident),
                            syn::Item::Use(u) => {
                                format!(
                                    "   - use {}",
                                    quote::quote!(#u).to_string().trim_end_matches(';')
                                )
                            }
                            _ => "   - (other item)".to_string(),
                        };
                        eprintln!("{}", item_desc);
                    }
                }
            }
        }

        // Run analysis
        match analyzer.analyze(&file_path) {
            Ok(violations) => {
                if verbose_level >= 1 && !violations.is_empty() {
                    eprintln!("   Found {} violation(s)", violations.len());
                }
                all_violations.extend(violations);
            }
            Err(e) => {
                eprintln!("   ERROR: {}", e);
            }
        }

        if verbose_level >= 1 {
            eprintln!();
        }
    }

    // Summary
    if verbose_level >= 1 {
        eprintln!("\nSummary:");
        eprintln!("  Files analyzed: {}", files_analyzed);
        if verbose_level >= 2 {
            eprintln!("  Total AST items: {}", total_items);
        }
        eprintln!("  Total violations: {}\n", all_violations.len());
    }

    // Sort violations by file path, then by code
    all_violations.sort_by(|a, b| {
        a.file_path
            .cmp(&b.file_path)
            .then_with(|| a.code.cmp(&b.code))
    });

    Ok(all_violations)
}
