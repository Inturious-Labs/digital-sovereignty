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
use ic_cdk::query;
// use ic_cdk::update; // TODO: uncomment when adding update endpoints

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
        // TODO: Add more status info (token count, session count, etc.)
    }
}

#[derive(CandidType, Deserialize)]
pub struct StatusResponse {
    pub version: String,
}

// Export Candid interface
ic_cdk::export_candid!();
