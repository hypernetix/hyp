//! E10 group checker registry.

use crate::{
    checker::Checker,
    checkers::e10::{
        E1001Config, E1001DirectPanic, E1002Config, E1002DirectUnwrapExpect, E1003Config,
        E1003UnsafeCode, E1004Config, E1004UnsafeWithoutComment, E1007Config,
        E1007NullPointerDeref, E1008Config, E1008UnsafeTraitImpl, E1010Config, E1010MutableStatic,
        E1013Config, E1013UnionFieldAccess, E1014Config, E1014RawPointerArithmetic, E1015Config,
        E1015UnwrapExpect, E1016Config, E1016MutexUnwrap, E1017Config, E1017TodoUnimplemented,
    },
    register_checker,
    registry::CheckerRegistration,
};

/// Get all E10 group checker registrations.
pub fn e10_registrations() -> Vec<CheckerRegistration> {
    vec![
        register_checker!(E1001DirectPanic, E1001Config),
        register_checker!(E1002DirectUnwrapExpect, E1002Config),
        register_checker!(E1003UnsafeCode, E1003Config),
        register_checker!(E1004UnsafeWithoutComment, E1004Config),
        register_checker!(E1007NullPointerDeref, E1007Config),
        register_checker!(E1008UnsafeTraitImpl, E1008Config),
        register_checker!(E1010MutableStatic, E1010Config),
        register_checker!(E1013UnionFieldAccess, E1013Config),
        register_checker!(E1014RawPointerArithmetic, E1014Config),
        register_checker!(E1015UnwrapExpect, E1015Config),
        register_checker!(E1016MutexUnwrap, E1016Config),
        register_checker!(E1017TodoUnimplemented, E1017Config),
    ]
}
