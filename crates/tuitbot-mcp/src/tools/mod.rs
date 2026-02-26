//! MCP tool implementations for Tuitbot.
//!
//! Shared tools live at this level; workflow-only tools are gated
//! behind the `workflow` submodule.

pub mod config;
pub mod idempotency;
#[allow(dead_code)]
pub mod manifest;
pub mod response;
pub mod scoring;

pub mod workflow;

#[cfg(test)]
mod benchmark;
#[cfg(test)]
mod boundary_tests;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod eval_harness;
