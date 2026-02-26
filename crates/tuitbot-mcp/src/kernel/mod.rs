//! Kernel layer: tool dispatch against provider traits.
//!
//! Tools in this layer depend only on [`SocialReadProvider`](crate::provider::SocialReadProvider)
//! and the contract envelope — never on `AppState`, `DbPool`, or concrete API clients.
//!
//! Write and engage modules take `&dyn XApiClient` directly (no provider trait yet)
//! for pragmatic DB-free operation in the API profile.

pub mod engage;
pub mod media;
pub mod read;
pub mod utils;
pub mod write;

#[cfg(test)]
mod tests;
