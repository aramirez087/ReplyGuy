//! Contract layer: protocol-level types reusable by any MCP consumer.
//!
//! Defines the response envelope, error taxonomy, and provider error mapping.
//! These types carry no TuitBot workflow assumptions.

pub mod envelope;
pub mod error;
pub mod error_code;

pub use envelope::{ToolError, ToolMeta, ToolResponse, WorkflowContext};
pub use error::ProviderError;
pub use error_code::ErrorCode;
