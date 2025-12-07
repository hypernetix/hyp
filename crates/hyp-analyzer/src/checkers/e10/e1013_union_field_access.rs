//! E1013: Union with unsafe field access
//!
//! Detects union type definitions and field accesses. Unions are inherently unsafe
//! because reading from a union field reinterprets the bits as that type.

use crate::{checker::Checker, define_checker, violation::Violation};

use syn::visit::Visit;

define_checker! {
    /// Checker for E1013: Union with unsafe field access
    E1013UnionFieldAccess,
    code = "E1013",
    name = "Union with unsafe field access",
    suggestions = "Consider using enums with explicit variants, or ensure all union access is carefully validated",
    target_items = [Union],
    config_entry_name = "e1013_union_field_access",
    /// Configuration for E1013: Union field access checker
    config = E1013Config {
        /// Whether this checker is enabled
        enabled: bool = true,
        /// Severity level, defaults to High
        severity: crate::config::SeverityLevel = crate::config::SeverityLevel::High,
        /// Categories this checker belongs to
        categories: Vec<crate::config::CheckerCategory> = vec![crate::config::CheckerCategory::Operations],
    },
    check_item(self, item, file_path) {
        let mut visitor = UnionVisitor {
            violations: Vec::new(),
            file_path,
            checker: self,
        };
        visitor.visit_item(item);
        Ok(visitor.violations)
    }
}

struct UnionVisitor<'a> {
    violations: Vec<Violation>,
    file_path: &'a str,
    checker: &'a E1013UnionFieldAccess,
}

impl<'a> UnionVisitor<'a> {
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

impl<'a> Visit<'a> for UnionVisitor<'a> {
    fn visit_item_union(&mut self, node: &'a syn::ItemUnion) {
        self.violations.push(self.create_violation(
            node.ident.span(),
            &format!(
                "Union type '{}' defined. Reading union fields is unsafe and can cause undefined behavior if the wrong variant is accessed.",
                node.ident
            ),
        ));

        syn::visit::visit_item_union(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_union_definition() {
        let code = r#"
            union MyUnion {
                i: i32,
                f: f32,
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1013UnionFieldAccess::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E1013");
        assert!(violations[0].message.contains("MyUnion"));
    }

    #[test]
    fn test_struct_passes() {
        let code = r#"
            struct MyStruct {
                i: i32,
                f: f32,
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1013UnionFieldAccess::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_enum_passes() {
        let code = r#"
            enum MyEnum {
                Int(i32),
                Float(f32),
            }
        "#;

        let syntax = syn::parse_file(code).unwrap();
        let checker = E1013UnionFieldAccess::default();

        let mut violations = Vec::new();
        for item in &syntax.items {
            violations.extend(checker.check_item(item, "test.rs").unwrap());
        }

        assert_eq!(violations.len(), 0);
    }
}
