//! Unified response envelope for MCP tools.
//!
//! Every MCP tool wraps its payload inside a [`ToolResponse`] envelope with
//! `success`, `data`, `error`, and `meta` fields. This envelope is
//! protocol-level and carries no TuitBot-specific assumptions.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::error_code::ErrorCode;

/// Unified envelope returned by MCP tools.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResponse {
    /// Whether the tool call succeeded.
    pub success: bool,
    /// The tool's payload (arbitrary JSON).
    pub data: Value,
    /// Present only on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ToolError>,
    /// Optional execution metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ToolMeta>,
}

/// Structured error information.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolError {
    /// Machine-readable error code.
    pub code: ErrorCode,
    /// Human-readable description.
    pub message: String,
    /// Whether the caller may retry the request.
    pub retryable: bool,
    /// Unix epoch or ISO-8601 timestamp when a rate limit resets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_reset: Option<String>,
    /// Policy decision label (e.g. `"denied"`, `"routed_to_approval"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_decision: Option<String>,
}

/// Workflow-specific context attached to metadata.
///
/// Flattened into [`ToolMeta`] so the JSON shape stays identical:
/// `{ "tool_version": "1.0", "elapsed_ms": 42, "mode": "autopilot", "approval_mode": false }`.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Operating mode (e.g. `"autopilot"`, `"composer"`).
    pub mode: String,
    /// Effective approval mode flag.
    pub approval_mode: bool,
}

/// Execution metadata attached to a tool response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolMeta {
    /// Envelope schema version.
    pub tool_version: String,
    /// Wall-clock execution time in milliseconds.
    pub elapsed_ms: u64,
    /// Workflow-specific fields (mode, approval_mode).
    /// Flattened so they appear as top-level keys in JSON.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowContext>,
}

impl ToolResponse {
    /// Build a success envelope wrapping any serializable payload.
    pub fn success(data: impl Serialize) -> Self {
        Self {
            success: true,
            data: serde_json::to_value(data).unwrap_or(Value::Null),
            error: None,
            meta: None,
        }
    }

    /// Build an error envelope. Retryable flag is derived from the error code.
    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: Value::Null,
            error: Some(ToolError {
                retryable: code.is_retryable(),
                code,
                message: message.into(),
                rate_limit_reset: None,
                policy_decision: None,
            }),
            meta: None,
        }
    }

    /// Convenience: database error (retryable).
    pub fn db_error(message: impl Into<String>) -> Self {
        Self::error(ErrorCode::DbError, message)
    }

    /// Convenience: validation error (not retryable).
    #[allow(dead_code)]
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::error(ErrorCode::ValidationError, message)
    }

    /// Convenience: LLM not configured (not retryable).
    pub fn llm_not_configured() -> Self {
        Self::error(
            ErrorCode::LlmNotConfigured,
            "LLM is not configured. Check your config.toml.",
        )
    }

    /// Convenience: X API not configured (not retryable).
    pub fn x_not_configured() -> Self {
        Self::error(
            ErrorCode::XNotConfigured,
            "X API client not available. Run `tuitbot auth` to authenticate.",
        )
    }

    /// Attach metadata to the response (builder pattern).
    pub fn with_meta(mut self, meta: ToolMeta) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Attach `rate_limit_reset` to the error payload (builder pattern).
    pub fn with_rate_limit_reset(mut self, reset: impl Into<String>) -> Self {
        if let Some(ref mut err) = self.error {
            err.rate_limit_reset = Some(reset.into());
        }
        self
    }

    /// Attach `policy_decision` to the error payload (builder pattern).
    pub fn with_policy_decision(mut self, decision: impl Into<String>) -> Self {
        if let Some(ref mut err) = self.error {
            err.policy_decision = Some(decision.into());
        }
        self
    }

    /// Serialize to a pretty-printed JSON string.
    ///
    /// Falls back to a minimal error JSON if serialization fails.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| {
            format!(
                r#"{{"success":false,"data":null,"error":{{"code":"serialization_error","message":"{}","retryable":false}}}}"#,
                e
            )
        })
    }
}

impl ToolMeta {
    /// Create metadata with just the elapsed time.
    pub fn new(elapsed_ms: u64) -> Self {
        Self {
            tool_version: "1.0".to_string(),
            elapsed_ms,
            workflow: None,
        }
    }

    /// Attach workflow context (mode + approval_mode) to metadata (builder pattern).
    pub fn with_workflow(mut self, mode: impl Into<String>, approval_mode: bool) -> Self {
        self.workflow = Some(WorkflowContext {
            mode: mode.into(),
            approval_mode,
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_envelope_shape() {
        let resp = ToolResponse::success(serde_json::json!({"count": 42}));
        assert!(resp.success);
        assert_eq!(resp.data["count"], 42);
        assert!(resp.error.is_none());
        assert!(resp.meta.is_none());
    }

    #[test]
    fn error_envelope_shape() {
        let resp = ToolResponse::error(ErrorCode::DbError, "connection refused");
        assert!(!resp.success);
        assert_eq!(resp.data, Value::Null);
        let err = resp.error.as_ref().unwrap();
        assert_eq!(err.code, ErrorCode::DbError);
        assert_eq!(err.message, "connection refused");
        assert!(err.retryable);
    }

    #[test]
    fn error_retryable_derived_from_code() {
        let resp = ToolResponse::error(ErrorCode::InvalidInput, "bad");
        assert!(!resp.error.as_ref().unwrap().retryable);

        let resp = ToolResponse::error(ErrorCode::XNetworkError, "timeout");
        assert!(resp.error.as_ref().unwrap().retryable);
    }

    #[test]
    fn meta_present_when_attached() {
        let meta = ToolMeta::new(123).with_workflow("autopilot", false);
        let resp = ToolResponse::success(serde_json::json!({})).with_meta(meta);
        let m = resp.meta.as_ref().unwrap();
        assert_eq!(m.elapsed_ms, 123);
        let wf = m.workflow.as_ref().unwrap();
        assert_eq!(wf.mode, "autopilot");
        assert!(!wf.approval_mode);
        assert_eq!(m.tool_version, "1.0");
    }

    #[test]
    fn meta_workflow_flattened_in_json() {
        let meta = ToolMeta::new(42).with_workflow("composer", true);
        let resp = ToolResponse::success(serde_json::json!({})).with_meta(meta);
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        // Flattened: mode and approval_mode appear at top level of meta
        assert_eq!(parsed["meta"]["mode"], "composer");
        assert_eq!(parsed["meta"]["approval_mode"], true);
        assert_eq!(parsed["meta"]["elapsed_ms"], 42);
    }

    #[test]
    fn meta_absent_by_default() {
        let json = ToolResponse::success(42).to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("meta").is_none());
    }

    #[test]
    fn meta_without_workflow_omits_mode() {
        let meta = ToolMeta::new(10);
        let resp = ToolResponse::success(1).with_meta(meta);
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["meta"].get("mode").is_none());
        assert!(parsed["meta"].get("approval_mode").is_none());
    }

    #[test]
    fn roundtrip_deserialization() {
        let resp = ToolResponse::success(serde_json::json!({"items": [1, 2, 3]}))
            .with_meta(ToolMeta::new(50));
        let json = resp.to_json();
        let back: ToolResponse = serde_json::from_str(&json).unwrap();
        assert!(back.success);
        assert_eq!(back.data["items"].as_array().unwrap().len(), 3);
        assert_eq!(back.meta.unwrap().elapsed_ms, 50);
    }

    #[test]
    fn typed_struct_as_data() {
        #[derive(Serialize)]
        struct Info {
            tier: String,
            count: u32,
        }
        let resp = ToolResponse::success(Info {
            tier: "pro".into(),
            count: 5,
        });
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["data"]["tier"], "pro");
        assert_eq!(parsed["data"]["count"], 5);
    }

    #[test]
    fn array_data() {
        let resp = ToolResponse::success(vec![1, 2, 3]);
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["data"].is_array());
        assert_eq!(parsed["data"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn rate_limit_reset_present_when_set() {
        let resp = ToolResponse::error(ErrorCode::XRateLimited, "too fast")
            .with_rate_limit_reset("2026-02-25T12:00:00Z");
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["error"]["rate_limit_reset"], "2026-02-25T12:00:00Z");
    }

    #[test]
    fn rate_limit_reset_absent_when_none() {
        let json = ToolResponse::error(ErrorCode::DbError, "fail").to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["error"].get("rate_limit_reset").is_none());
    }

    #[test]
    fn policy_decision_present_when_set() {
        let resp = ToolResponse::error(ErrorCode::PolicyDeniedBlocked, "blocked")
            .with_policy_decision("denied");
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["error"]["policy_decision"], "denied");
    }

    #[test]
    fn policy_decision_absent_when_none() {
        let json = ToolResponse::error(ErrorCode::DbError, "fail").to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["error"].get("policy_decision").is_none());
    }

    #[test]
    fn db_error_constructor() {
        let resp = ToolResponse::db_error("connection refused");
        assert!(!resp.success);
        let err = resp.error.as_ref().unwrap();
        assert_eq!(err.code, ErrorCode::DbError);
        assert!(err.retryable);
    }

    #[test]
    fn validation_error_constructor() {
        let resp = ToolResponse::validation_error("missing field");
        let err = resp.error.as_ref().unwrap();
        assert_eq!(err.code, ErrorCode::ValidationError);
        assert!(!err.retryable);
    }

    #[test]
    fn llm_not_configured_constructor() {
        let resp = ToolResponse::llm_not_configured();
        let err = resp.error.as_ref().unwrap();
        assert_eq!(err.code, ErrorCode::LlmNotConfigured);
        assert!(!err.retryable);
    }

    #[test]
    fn x_not_configured_constructor() {
        let resp = ToolResponse::x_not_configured();
        let err = resp.error.as_ref().unwrap();
        assert_eq!(err.code, ErrorCode::XNotConfigured);
        assert!(!err.retryable);
    }

    #[test]
    fn error_code_serializes_as_string_in_json() {
        let resp = ToolResponse::error(ErrorCode::DbError, "fail");
        let json = resp.to_json();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["error"]["code"], "db_error");
    }

    #[test]
    fn builders_no_op_on_success() {
        let resp = ToolResponse::success(42)
            .with_rate_limit_reset("never")
            .with_policy_decision("none");
        assert!(resp.error.is_none());
    }
}
