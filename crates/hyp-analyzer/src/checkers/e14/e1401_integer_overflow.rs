//! E1401: Integer overflow/underflow
//!
//! Detects unchecked arithmetic operations that can overflow in release builds.
//! In release mode, Rust wraps on overflow which causes subtle bugs.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::{spanned::Spanned, visit::Visit, BinOp};

define_checker! {
    /// Checker for E1401: Integer overflow/underflow
    E1401IntegerOverflow,
    code = "E1401",
    name = "Integer overflow/underflow",
    suggestions = "Use checked_add(), saturating_add(), or wrapping_add() for explicit overflow handling.",
    target_items = [Function],
    config_entry_name = "e1401_integer_overflow",
    /// Configuration for E1401: Integer overflow checker
    config = E1401Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    // AST node item checker
    check_item(self, item, file_path) {
        let mut visitor = OverflowVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

/// Visitor that looks for unchecked arithmetic operations
struct OverflowVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1401IntegerOverflow,
}

impl<'a> OverflowVisitor<'a> {
    fn create_violation(&self, span: proc_macro2::Span, op: &str) -> Violation {
        let start = span.start();
        Violation::new(
            self.checker.code(),
            self.checker.name(),
            self.checker.severity().into(),
            &format!(
                "Unchecked {} operation can overflow in release builds, causing wraparound behavior.",
                op
            ),
            self.file_path,
            start.line,
            start.column + 1,
        )
        .with_suggestion(self.checker.suggestions())
    }
}

impl<'a> Visit<'a> for OverflowVisitor<'a> {
    fn visit_expr_binary(&mut self, node: &'a syn::ExprBinary) {
        let op_name = match &node.op {
            BinOp::Add(_) => Some("addition"),
            BinOp::Sub(_) => Some("subtraction"),
            BinOp::Mul(_) => Some("multiplication"),
            _ => None,
        };

        if let Some(op) = op_name {
            // Check if either operand involves a literal that could cause overflow
            // This is a simplified check - a more sophisticated version would do type inference
            if contains_integer_literal(&node.left) || contains_integer_literal(&node.right) {
                self.violations.push(self.create_violation(node.span(), op));
            }
        }

        // Continue visiting nested expressions
        syn::visit::visit_expr_binary(self, node);
    }
}

/// Check if an expression contains an integer literal
fn contains_integer_literal(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(lit) => matches!(lit.lit, syn::Lit::Int(_)),
        syn::Expr::Binary(bin) => {
            contains_integer_literal(&bin.left) || contains_integer_literal(&bin.right)
        }
        syn::Expr::Paren(paren) => contains_integer_literal(&paren.expr),
        syn::Expr::Group(group) => contains_integer_literal(&group.expr),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_addition_overflow() {
        let code = r#"
            fn example(x: u8) -> u8 {
                x + 100
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1401IntegerOverflow::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1401");
        assert!(violations[0].message.contains("addition"));
    }

    #[test]
    fn test_detects_subtraction_overflow() {
        let code = r#"
            fn example(x: u8) -> u8 {
                x - 50
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1401IntegerOverflow::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("subtraction"));
    }

    #[test]
    fn test_detects_multiplication_overflow() {
        let code = r#"
            fn example(x: u8) -> u8 {
                x * 10
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1401IntegerOverflow::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("multiplication"));
    }

    #[test]
    fn test_checked_add_passes() {
        let code = r#"
            fn example(x: u8) -> Option<u8> {
                x.checked_add(100)
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1401IntegerOverflow::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }
}
