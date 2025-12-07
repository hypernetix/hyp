//! E15 group checker registry.

use crate::{
    checker::Checker,
    checkers::e15::{
        E1502Config, E1502LockAcrossAwait, E1503Config, E1503LockPoisoning, E1506Config,
        E1506DeadlockLockOrdering, E1508Config, E1508SleepInsteadOfSync, E1509Config,
        E1509ChannelLifetime, E1510Config, E1510MutexInsteadOfRwLock,
    },
    register_checker,
    registry::CheckerRegistration,
};

/// Get all E15 group checker registrations.
pub fn e15_registrations() -> Vec<CheckerRegistration> {
    vec![
        register_checker!(E1502LockAcrossAwait, E1502Config),
        register_checker!(E1503LockPoisoning, E1503Config),
        register_checker!(E1506DeadlockLockOrdering, E1506Config),
        register_checker!(E1508SleepInsteadOfSync, E1508Config),
        register_checker!(E1509ChannelLifetime, E1509Config),
        register_checker!(E1510MutexInsteadOfRwLock, E1510Config),
    ]
}
