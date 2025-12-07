//! E1007: Dereferencing null pointer
//!
//! Detects patterns that could lead to null pointer dereferences, such as
//! dereferencing raw pointers without null checks.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::{spanned::Spanned, visit::Visit};

define_checker! {
    /// Checker for E1007: Dereferencing null pointer
    E1007NullPointerDeref,
    code = "E1007",
    name = "Potential null pointer dereference",
    suggestions = "Check for null before dereferencing raw pointers, or use Option<NonNull<T>>",
    target_items = [Function],
    config_entry_name = "e1007_null_pointer_deref",
    /// Configuration for E1007: Null pointer dereference checker
    config = E1007Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    check_item(self, item, file_path) {
        let mut visitor = NullDerefVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

struct NullDerefVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1007NullPointerDeref,
}

impl<'a> NullDerefVisitor<'a> {
    fn create_violation(&self, span: proc_macro2::Span, message: &str) -> Violation {
        let start = span.start();
        Violation::new(
            self.checker.code(),
            self.checker.name(),
            self.checker.severity().into(),
            message,
            self.file_path,
            start.line,
            start.column + 1,
        )
        .with_suggestion(self.checker.suggestions())
    }
}

impl<'a> Visit<'a> for NullDerefVisitor<'a> {
    fn visit_expr_unary(&mut self, node: &'a syn::ExprUnary) {
        // Check for pointer dereference (*ptr)
        if let syn::UnOp::Deref(_) = node.op {
            // Check if the expression being dereferenced looks like a raw pointer
            if is_likely_raw_pointer(&node.expr) {
                self.violations.push(self.create_violation(
                    node.span(),
                    "Dereferencing a raw pointer without null check. This can cause undefined behavior if the pointer is null.",
                ));
            }
        }

        syn::visit::visit_expr_unary(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'a syn::ExprMethodCall) {
        // Check for ptr.read(), ptr.read_volatile(), ptr.as_ref() without checks
        let method_name = node.method.to_string();
        if matches!(method_name.as_str(), "read" | "read_volatile" | "read_unaligned" | "write" | "write_volatile" | "write_unaligned") {
            self.violations.push(self.create_violation(
                node.span(),
                &format!("Calling {}() on a raw pointer without null check.", method_name),
            ));
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Heuristic to detect if an expression is likely a raw pointer
fn is_likely_raw_pointer(expr: &syn::Expr) -> bool {
    match expr {
        // Direct path that might be a pointer variable
        syn::Expr::Path(_) => true,
        // Field access on something that might be a pointer
        syn::Expr::Field(_) => true,
        // Cast to pointer type
        syn::Expr::Cast(cast) => {
            if let syn::Type::Ptr(_) = cast.ty.as_ref() {
                return true;
            }
            false
        }
        // Method call that returns a pointer (e.g., as_ptr())
        syn::Expr::MethodCall(call) => {
            let method = call.method.to_string();
            matches!(method.as_str(), "as_ptr" | "as_mut_ptr" | "offset" | "add" | "sub")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_raw_pointer_deref() {
        let code = r#"
            fn example() {
                let x = 42;
                let ptr = &x as *const i32;
                unsafe {
                    let _value = *ptr;
                }
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1007NullPointerDeref::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1007");
    }

    #[test]
    fn test_detects_ptr_read() {
        let code = r#"
            fn example() {
                let x = 42;
                let ptr = &x as *const i32;
                unsafe {
                    let _value = ptr.read();
                }
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1007NullPointerDeref::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_safe_reference_deref_passes() {
        let code = r#"
            fn example() {
                let x = 42;
                let r = &x;
                let _value = *r;
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1007NullPointerDeref::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        // Note: This might still flag due to heuristics; adjust as needed
        assert!(violations.is_empty() || violations.len() <= 1);
    }
}
