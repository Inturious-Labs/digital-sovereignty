//! Persistent storage for tokens, sessions, and gifts
//!
//! Uses IC stable structures for data that survives canister upgrades.

use candid::{CandidType, Deserialize};
use serde::Serialize;

/// Access token issued after successful payment
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AccessToken {
    pub token: String,
    pub email: String,
    pub article_slug: String,
    pub created_at: u64,
    pub expires_at: u64,
}

/// User session for browser-based access
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserSession {
    pub session_id: String,
    pub email: String,
    pub paid_articles: Vec<String>,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Gift token for sharing articles
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GiftToken {
    pub gift_token: String,
    pub article_slug: String,
    pub gifter_email: String,
    pub recipient_email: Option<String>,
    pub redeemed: bool,
    pub redeemed_by: Option<String>,
    pub created_at: u64,
    pub expires_at: u64,
}

// TODO: Implement stable storage using ic-stable-structures
// - BTreeMap for tokens (token_id -> AccessToken)
// - BTreeMap for sessions (session_id -> UserSession)
// - BTreeMap for gifts (gift_token -> GiftToken)
// - Index maps for lookups (email -> tokens, article -> tokens)
