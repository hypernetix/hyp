//! E1402: Division by zero
//!
//! Detects division operations that don't check for zero divisors.
//! Division by zero causes a runtime panic.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::{spanned::Spanned, visit::Visit, BinOp};

define_checker! {
    /// Checker for E1402: Division by zero
    E1402DivisionByZero,
    code = "E1402",
    name = "Division by zero",
    suggestions = "Use checked_div() which returns None for division by zero, or validate the divisor before dividing.",
    target_items = [Function],
    config_entry_name = "e1402_division_by_zero",
    /// Configuration for E1402: Division by zero checker
    config = E1402Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    // AST node item checker
    check_item(self, item, file_path) {
        let mut visitor = DivisionVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

/// Visitor that looks for division operations
struct DivisionVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1402DivisionByZero,
}

impl<'a> DivisionVisitor<'a> {
    fn create_violation(&self, span: proc_macro2::Span) -> Violation {
        let start = span.start();
        Violation::new(
            self.checker.code(),
            self.checker.name(),
            self.checker.severity().into(),
            "Division operation without zero check. Will panic if divisor is zero.",
            self.file_path,
            start.line,
            start.column + 1,
        )
        .with_suggestion(self.checker.suggestions())
    }
}

impl<'a> Visit<'a> for DivisionVisitor<'a> {
    fn visit_expr_binary(&mut self, node: &'a syn::ExprBinary) {
        if matches!(node.op, BinOp::Div(_)) {
            // Check if the divisor is a non-zero literal (safe case)
            if !is_non_zero_literal(&node.right) {
                self.violations.push(self.create_violation(node.span()));
            }
        }

        // Continue visiting nested expressions
        syn::visit::visit_expr_binary(self, node);
    }
}

/// Check if an expression is a non-zero integer literal
fn is_non_zero_literal(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Int(int_lit) = &lit.lit {
                // Parse the literal value and check if it's non-zero
                int_lit
                    .base10_parse::<i128>()
                    .map(|v| v != 0)
                    .unwrap_or(false)
            } else {
                false
            }
        }
        syn::Expr::Paren(paren) => is_non_zero_literal(&paren.expr),
        syn::Expr::Group(group) => is_non_zero_literal(&group.expr),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_division_by_variable() {
        let code = r#"
            fn example(x: i32, y: i32) -> i32 {
                x / y
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1402DivisionByZero::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1402");
    }

    #[test]
    fn test_allows_division_by_non_zero_literal() {
        let code = r#"
            fn example(x: i32) -> i32 {
                x / 2
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1402DivisionByZero::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_detects_division_by_zero_literal() {
        let code = r#"
            fn example(x: i32) -> i32 {
                x / 0
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1402DivisionByZero::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        // Division by 0 literal is still flagged (not a non-zero literal)
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_no_division_passes() {
        let code = r#"
            fn example(x: i32, y: i32) -> i32 {
                x + y
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1402DivisionByZero::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }
}
