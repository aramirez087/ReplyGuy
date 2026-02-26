//! MCP server implementations.
//!
//! - [`TuitbotMcpServer`]: full workflow profile (all 60+ tools, requires DB).
//! - [`ApiMcpServer`]: lightweight API profile (~24 tools, no DB).

pub mod api;
pub mod workflow;

pub use api::ApiMcpServer;
pub use workflow::TuitbotMcpServer;
