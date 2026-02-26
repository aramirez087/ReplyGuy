//! Kernel layer: tool dispatch against provider traits.
//!
//! Tools in this layer depend only on [`SocialReadProvider`](crate::provider::SocialReadProvider)
//! and the contract envelope — never on `AppState`, `DbPool`, or concrete API clients.
//!
//! Write and engage modules take `&dyn XApiClient` directly (no provider trait yet)
//! for pragmatic DB-free operation in the API profile.

// Engage, media, and write kernels are currently only exercised by
// conformance tests (the read-only profile servers don't use them).
// Kept as the canonical kernel abstraction for future profiles.
#[allow(dead_code)]
pub mod engage;
#[allow(dead_code)]
pub mod media;
pub mod read;
pub mod utils;
#[allow(dead_code)]
pub mod write;

#[cfg(test)]
mod tests;
