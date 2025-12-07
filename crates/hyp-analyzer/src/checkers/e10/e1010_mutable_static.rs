//! E1010: Mutable static without synchronization
//!
//! Detects mutable static variables which are inherently unsafe and can cause
//! data races in multi-threaded programs.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::visit::Visit;

define_checker! {
    /// Checker for E1010: Mutable static without synchronization
    E1010MutableStatic,
    code = "E1010",
    name = "Mutable static without synchronization",
    suggestions = "Use thread-safe alternatives like Mutex<T>, RwLock<T>, AtomicXxx, or lazy_static!/once_cell",
    target_items = [Static],
    config_entry_name = "e1010_mutable_static",
    /// Configuration for E1010: Mutable static checker
    config = E1010Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    check_item(self, item, file_path) {
        let mut visitor = MutableStaticVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

struct MutableStaticVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1010MutableStatic,
}

impl<'a> MutableStaticVisitor<'a> {
    fn create_violation(&self, span: proc_macro2::Span, name: &str) -> Violation {
        let start = span.start();
        Violation::new(
            self.checker.code(),
            self.checker.name(),
            self.checker.severity().into(),
            &format!(
                "Mutable static '{}' is inherently unsafe and can cause data races. Access requires unsafe blocks.",
                name
            ),
            self.file_path,
            start.line,
            start.column + 1,
        )
        .with_suggestion(self.checker.suggestions())
    }
}

impl<'a> Visit<'a> for MutableStaticVisitor<'a> {
    fn visit_item_static(&mut self, node: &'a syn::ItemStatic) {
        // Check if the static is mutable
        if matches!(node.mutability, syn::StaticMutability::Mut(_)) {
            self.violations.push(self.create_violation(
                node.ident.span(),
                &node.ident.to_string(),
            ));
        }

        syn::visit::visit_item_static(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_mutable_static() {
        let code = r#"
            static mut COUNTER: i32 = 0;
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1010MutableStatic::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1010");
        assert!(violations[0].message.contains("COUNTER"));
    }

    #[test]
    fn test_immutable_static_passes() {
        let code = r#"
            static CONSTANT: i32 = 42;
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1010MutableStatic::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_const_passes() {
        let code = r#"
            const VALUE: i32 = 42;
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1010MutableStatic::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }
}
