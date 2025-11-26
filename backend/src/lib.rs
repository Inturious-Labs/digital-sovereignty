//! Digital Sovereignty Chronicle - Paywall Backend Canister
//!
//! This canister handles:
//! - Stripe payment processing
//! - Access token generation and validation
//! - User session management
//! - Gift token creation and redemption
//! - Email notifications via HTTP outcalls

mod auth;
mod email;
mod storage;
mod stripe;

use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};

/// Health check endpoint
#[query]
fn health() -> String {
    "OK".to_string()
}

/// Get canister status information
#[query]
fn status() -> StatusResponse {
    StatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        access_tokens: storage::count_access_tokens(),
        user_sessions: storage::count_user_sessions(),
        gift_tokens: storage::count_gift_tokens(),
    }
}

#[derive(CandidType, Deserialize)]
pub struct StatusResponse {
    pub version: String,
    pub access_tokens: u64,
    pub user_sessions: u64,
    pub gift_tokens: u64,
}

// ============================================================================
// Test endpoints (for validating storage works)
// ============================================================================

/// Insert a test access token (for testing storage)
#[update]
fn test_insert_token(email: String, article_slug: String) -> String {
    let token = format!("test_token_{}", ic_cdk::api::time());
    let access_token = storage::AccessToken {
        token: token.clone(),
        email,
        article_slug,
        created_at: ic_cdk::api::time(),
        expires_at: ic_cdk::api::time() + 365 * 24 * 60 * 60 * 1_000_000_000, // 1 year in nanoseconds
    };
    storage::insert_access_token(access_token);
    token
}

/// Retrieve a test access token (for testing storage)
#[query]
fn test_get_token(token: String) -> Option<storage::AccessToken> {
    storage::get_access_token(&token)
}

// Export Candid interface
ic_cdk::export_candid!();
