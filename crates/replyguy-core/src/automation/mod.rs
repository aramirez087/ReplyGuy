//! Automation loops for tweet discovery and mentions monitoring.
//!
//! This module contains the core engagement engine that delivers the
//! primary value proposition of ReplyGuy: autonomous tweet discovery
//! and contextual reply generation.
//!
//! Submodules:
//! - [`loop_helpers`]: Shared types, traits, and error handling for loops.
//! - [`mentions_loop`]: Monitors @-mentions and generates replies.
//! - [`discovery_loop`]: Searches tweets by keyword, scores, and replies.

pub mod discovery_loop;
pub mod loop_helpers;
pub mod mentions_loop;

pub use discovery_loop::{DiscoveryLoop, DiscoveryResult, DiscoverySummary};
pub use loop_helpers::{
    ConsecutiveErrorTracker, LoopError, LoopStorage, LoopTweet, MentionsFetcher, PostSender,
    ReplyGenerator, SafetyChecker, ScoreResult, TweetScorer, TweetSearcher,
};
pub use mentions_loop::{MentionResult, MentionsLoop};
