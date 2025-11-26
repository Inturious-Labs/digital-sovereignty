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

/// Insert a test access token (for testing storage - uses simple token)
#[update]
fn test_insert_token(email: String, article_slug: String) -> String {
    let token = format!("test_token_{}", ic_cdk::api::time());
    let access_token = storage::AccessToken {
        token: token.clone(),
        email,
        article_slug,
        created_at: ic_cdk::api::time(),
        expires_at: ic_cdk::api::time() + 365 * 24 * 60 * 60 * 1_000_000_000,
    };
    storage::insert_access_token(access_token);
    token
}

/// Retrieve a test access token (for testing storage)
#[query]
fn test_get_token(token: String) -> Option<storage::AccessToken> {
    storage::get_access_token(&token)
}

// ============================================================================
// Auth test endpoints (for validating auth module)
// ============================================================================

/// Create access token using secure random generation
#[update]
async fn auth_create_token(email: String, article_slug: String) -> AuthCreateTokenResponse {
    match auth::create_access_token(email, article_slug).await {
        Ok(token) => AuthCreateTokenResponse {
            success: true,
            token: Some(token.token),
            error: None,
        },
        Err(e) => AuthCreateTokenResponse {
            success: false,
            token: None,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct AuthCreateTokenResponse {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

/// Validate an access token
#[query]
fn auth_validate_token(token: String) -> AuthValidateResponse {
    match auth::validate_access_token(&token) {
        Ok(access_token) => AuthValidateResponse {
            valid: true,
            article_slug: Some(access_token.article_slug),
            email: Some(access_token.email),
            error: None,
        },
        Err(e) => AuthValidateResponse {
            valid: false,
            article_slug: None,
            email: None,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct AuthValidateResponse {
    pub valid: bool,
    pub article_slug: Option<String>,
    pub email: Option<String>,
    pub error: Option<String>,
}

/// Check if token grants access to specific article
#[query]
fn auth_check_access(token: String, article_slug: String) -> bool {
    auth::check_article_access(&article_slug, Some(&token), None)
}

/// Test HMAC signing
#[query]
fn auth_test_signing(data: String) -> SigningTestResponse {
    let signature = auth::sign_data(&data);
    let cookie = auth::create_signed_cookie(&data);
    let verified = auth::verify_signature(&data, &signature);

    SigningTestResponse {
        signature,
        signed_cookie: cookie,
        signature_valid: verified,
    }
}

#[derive(CandidType, Deserialize)]
pub struct SigningTestResponse {
    pub signature: String,
    pub signed_cookie: String,
    pub signature_valid: bool,
}

// Export Candid interface
ic_cdk::export_candid!();
