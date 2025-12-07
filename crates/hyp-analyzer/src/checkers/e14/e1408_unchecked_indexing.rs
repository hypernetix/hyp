//! E1408: Unchecked array indexing
//!
//! Detects direct array/slice indexing with `[]` which can panic at runtime
//! if the index is out of bounds.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::{spanned::Spanned, visit::Visit};

define_checker! {
    /// Checker for E1408: Unchecked array indexing
    E1408UncheckedIndexing,
    code = "E1408",
    name = "Unchecked array indexing",
    suggestions = "Use .get() for fallible access, or validate the index before using []",
    target_items = [Function],
    config_entry_name = "e1408_unchecked_indexing",
    /// Configuration for E1408: Unchecked indexing checker
    config = E1408Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    check_item(self, item, file_path) {
        let mut visitor = IndexingVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

struct IndexingVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1408UncheckedIndexing,
}

impl<'a> IndexingVisitor<'a> {
    fn create_violation(&self, span: proc_macro2::Span) -> Violation {
        let start = span.start();
        Violation::new(
            self.checker.code(),
            self.checker.name(),
            self.checker.severity().into(),
            "Direct indexing with [] can panic if the index is out of bounds.",
            self.file_path,
            start.line,
            start.column + 1,
        )
        .with_suggestion(self.checker.suggestions())
    }
}

impl<'a> Visit<'a> for IndexingVisitor<'a> {
    fn visit_expr_index(&mut self, node: &'a syn::ExprIndex) {
        // Check if the index is a variable (not a constant)
        // Constants are often safe (e.g., arr[0] on a known non-empty array)
        if !is_constant_index(&node.index) {
            self.violations.push(self.create_violation(node.span()));
        }

        syn::visit::visit_expr_index(self, node);
    }
}

/// Check if an index expression is a compile-time constant
fn is_constant_index(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(lit) => matches!(lit.lit, syn::Lit::Int(_)),
        syn::Expr::Path(path) => {
            // Check if it's a const (uppercase by convention)
            if let Some(ident) = path.path.get_ident() {
                let name = ident.to_string();
                name.chars().all(|c| c.is_uppercase() || c == '_')
            } else {
                false
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_variable_index() {
        let code = r#"
            fn example(arr: &[i32], idx: usize) -> i32 {
                arr[idx]
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1408UncheckedIndexing::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1408");
    }

    #[test]
    fn test_constant_index_passes() {
        let code = r#"
            fn example(arr: &[i32; 5]) -> i32 {
                arr[0]
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1408UncheckedIndexing::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_get_method_passes() {
        let code = r#"
            fn example(arr: &[i32], idx: usize) -> Option<&i32> {
                arr.get(idx)
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1408UncheckedIndexing::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_detects_expression_index() {
        let code = r#"
            fn example(arr: &[i32], a: usize, b: usize) -> i32 {
                arr[a + b]
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1408UncheckedIndexing::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
    }
}
