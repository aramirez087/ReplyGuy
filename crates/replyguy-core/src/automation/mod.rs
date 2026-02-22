//! Automation loops for content posting and thread generation.
//!
//! This module contains the content and thread loops that keep
//! the user's X account active with original educational content.
//!
//! Submodules:
//! - [`loop_helpers`]: Shared types, traits, and error handling.
//! - [`content_loop`]: Generates and posts educational tweets.
//! - [`thread_loop`]: Generates and posts multi-tweet threads.

pub mod content_loop;
pub mod loop_helpers;
pub mod thread_loop;

pub use content_loop::{ContentLoop, ContentResult};
pub use loop_helpers::{
    ContentLoopError, ContentSafety, ContentStorage, ThreadPoster, TweetGenerator,
};
pub use thread_loop::{ThreadGenerator, ThreadLoop, ThreadResult};
