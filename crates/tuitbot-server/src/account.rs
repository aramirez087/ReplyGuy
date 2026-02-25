//! Account context extraction and role-based access control.
//!
//! Resolves the `X-Account-Id` header into an `AccountContext` with the
//! caller's role. Missing header defaults to the backward-compatible
//! default account.

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tuitbot_core::storage::accounts::{self, DEFAULT_ACCOUNT_ID};

use crate::state::AppState;

/// Resolved account context available to route handlers.
#[derive(Debug, Clone)]
pub struct AccountContext {
    /// The account ID (UUIDv4 or default sentinel).
    pub account_id: String,
    /// The caller's role on this account.
    pub role: Role,
}

/// Role tiers for account access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Approver,
    Viewer,
}

impl Role {
    /// Whether this role can perform read operations (always true).
    pub fn can_read(self) -> bool {
        true
    }

    /// Whether this role can approve/reject items.
    pub fn can_approve(self) -> bool {
        matches!(self, Role::Admin | Role::Approver)
    }

    /// Whether this role can perform mutations (config, runtime, compose).
    pub fn can_mutate(self) -> bool {
        matches!(self, Role::Admin)
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Approver => write!(f, "approver"),
            Role::Viewer => write!(f, "viewer"),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "approver" => Ok(Role::Approver),
            "viewer" => Ok(Role::Viewer),
            other => Err(format!("unknown role: {other}")),
        }
    }
}

/// Error returned when account context extraction fails.
pub struct AccountError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        (self.status, axum::Json(json!({"error": self.message}))).into_response()
    }
}

impl FromRequestParts<Arc<AppState>> for AccountContext {
    type Rejection = AccountError;

    /// Extract account context from the `X-Account-Id` header.
    ///
    /// - Missing header → default account with admin role (backward compat).
    /// - Present header → validates account exists and resolves role.
    fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let account_id = parts
            .headers
            .get("x-account-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or(DEFAULT_ACCOUNT_ID)
            .to_string();

        let db = state.db.clone();

        async move {
            // Default account always grants admin.
            if account_id == DEFAULT_ACCOUNT_ID {
                return Ok(AccountContext {
                    account_id,
                    role: Role::Admin,
                });
            }

            // Validate account exists and is active.
            let exists = accounts::account_exists(&db, &account_id)
                .await
                .map_err(|e| AccountError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("failed to validate account: {e}"),
                })?;

            if !exists {
                return Err(AccountError {
                    status: StatusCode::NOT_FOUND,
                    message: format!("account not found: {account_id}"),
                });
            }

            // Resolve role — default actor is "dashboard" for HTTP requests.
            let role_str = accounts::get_role(&db, &account_id, "dashboard")
                .await
                .map_err(|e| AccountError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("failed to resolve role: {e}"),
                })?;

            let role = role_str
                .as_deref()
                .unwrap_or("viewer")
                .parse::<Role>()
                .unwrap_or(Role::Viewer);

            Ok(AccountContext { account_id, role })
        }
    }
}

/// Helper to reject requests that require approval permissions.
pub fn require_approve(ctx: &AccountContext) -> Result<(), AccountError> {
    if ctx.role.can_approve() {
        Ok(())
    } else {
        Err(AccountError {
            status: StatusCode::FORBIDDEN,
            message: "approver or admin role required".to_string(),
        })
    }
}

/// Helper to reject requests that require mutation permissions.
pub fn require_mutate(ctx: &AccountContext) -> Result<(), AccountError> {
    if ctx.role.can_mutate() {
        Ok(())
    } else {
        Err(AccountError {
            status: StatusCode::FORBIDDEN,
            message: "admin role required".to_string(),
        })
    }
}
