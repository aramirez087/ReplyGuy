//! MCP server implementations.
//!
//! - [`TuitbotMcpServer`]: full workflow profile (all 60+ tools, requires DB).
//! - [`ReadonlyMcpServer`]: minimal readonly profile (10 tools, no DB).
//! - [`ApiReadonlyMcpServer`]: broader api-readonly profile (20 tools, no DB).

pub mod api_readonly;
pub mod readonly;
pub mod workflow;

pub use api_readonly::ApiReadonlyMcpServer;
pub use readonly::ReadonlyMcpServer;
pub use workflow::TuitbotMcpServer;
