//! Shared, stable contract error codes (PRD §8 / ARCHITECTURE.md: release wasm strips panic
//! strings, so a trapped call otherwise shows only `UnreachableCodeReached`). Typed errors are part
//! of the ABI: operators, `services/api` and the console can map a code to a cause.
//!
//! Discriminants are stable and must never be renumbered or reused; add new variants at the end.
//! `ballast-common` raises these via `panic_with_error!`; individual contracts adopt them in
//! follow-up work (see the error-code table in `contracts/README.md`).

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// A required role/config entry has not been initialized yet.
    NotInitialized = 1,
    /// `initialize` (or another one-shot setup) was called a second time.
    AlreadyInitialized = 2,
    /// The action is blocked by an active pause scope (or the `All` override).
    Paused = 3,
    /// A timelocked change was executed before its delay elapsed.
    TimelockNotElapsed = 4,
    /// No entry exists for the supplied id/key (unknown, executed or cancelled).
    NotFound = 5,
    /// An argument is out of range or otherwise unusable.
    InvalidInput = 6,
    /// The caller is not authorized for this action.
    Unauthorized = 7,
}
