//! Authentication and token management
//!
//! Handles:
//! - Secure token generation using IC random beacon
//! - Token validation and expiration checking
//! - Session creation and management
//! - HMAC signing for session cookies

use crate::storage::{
    self, AccessToken, GiftToken, UserSession,
};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::cell::RefCell;

type HmacSha256 = Hmac<Sha256>;

// Configuration constants
const TOKEN_EXPIRY_DAYS: u64 = 365;
const SESSION_EXPIRY_DAYS: u64 = 30;
const GIFT_EXPIRY_DAYS: u64 = 90;
const NANOS_PER_DAY: u64 = 24 * 60 * 60 * 1_000_000_000;

// HMAC secret key for signing cookies/tokens
// Configure via auth_configure() canister call - default is insecure placeholder
thread_local! {
    static HMAC_SECRET: RefCell<Vec<u8>> = RefCell::new(b"default_secret_change_in_production".to_vec());
}

/// Result type for auth operations
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication errors
#[derive(Debug, Clone)]
pub enum AuthError {
    TokenNotFound,
    TokenExpired,
    SessionNotFound,
    SessionExpired,
    GiftNotFound,
    GiftAlreadyRedeemed,
    GiftExpired,
    InvalidSignature,
    RandomGenerationFailed,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::TokenNotFound => write!(f, "Access token not found"),
            AuthError::TokenExpired => write!(f, "Access token has expired"),
            AuthError::SessionNotFound => write!(f, "Session not found"),
            AuthError::SessionExpired => write!(f, "Session has expired"),
            AuthError::GiftNotFound => write!(f, "Gift token not found"),
            AuthError::GiftAlreadyRedeemed => write!(f, "Gift has already been redeemed"),
            AuthError::GiftExpired => write!(f, "Gift token has expired"),
            AuthError::InvalidSignature => write!(f, "Invalid signature"),
            AuthError::RandomGenerationFailed => write!(f, "Failed to generate random bytes"),
        }
    }
}

// ============================================================================
// Token Generation
// ============================================================================

/// Generate a cryptographically secure random token using IC's random beacon
pub async fn generate_secure_token() -> AuthResult<String> {
    let random_bytes = ic_cdk::management_canister::raw_rand()
        .await
        .map_err(|_| AuthError::RandomGenerationFailed)?;

    // Take first 32 bytes and encode as hex (64 characters)
    Ok(hex::encode(&random_bytes[..32]))
}

/// Generate a token synchronously using timestamp + caller (less secure, for fallback)
pub fn generate_fallback_token() -> String {
    let time = ic_cdk::api::time();
    let caller = ic_cdk::api::msg_caller();
    let data = format!("{:?}{}", caller, time);
    hex::encode(sha2::Sha256::digest(data.as_bytes()))
}

// ============================================================================
// Access Token Operations
// ============================================================================

/// Create a new access token after successful payment
pub async fn create_access_token(email: String, article_slug: String) -> AuthResult<AccessToken> {
    let token = generate_secure_token().await?;
    let now = ic_cdk::api::time();

    let access_token = AccessToken {
        token: token.clone(),
        email,
        article_slug,
        created_at: now,
        expires_at: now + (TOKEN_EXPIRY_DAYS * NANOS_PER_DAY),
    };

    storage::insert_access_token(access_token.clone());
    Ok(access_token)
}

/// Validate an access token and check expiration
pub fn validate_access_token(token_id: &str) -> AuthResult<AccessToken> {
    let token = storage::get_access_token(token_id)
        .ok_or(AuthError::TokenNotFound)?;

    let now = ic_cdk::api::time();
    if now > token.expires_at {
        return Err(AuthError::TokenExpired);
    }

    Ok(token)
}

/// Check if a token grants access to a specific article
pub fn token_grants_access(token_id: &str, article_slug: &str) -> AuthResult<bool> {
    let token = validate_access_token(token_id)?;
    Ok(token.article_slug == article_slug)
}

// ============================================================================
// Session Management
// ============================================================================

/// Create a new session for a user
pub async fn create_session(email: String, initial_article: Option<String>) -> AuthResult<UserSession> {
    let session_id = generate_secure_token().await?;
    let now = ic_cdk::api::time();

    let paid_articles = match initial_article {
        Some(article) => vec![article],
        None => vec![],
    };

    let session = UserSession {
        session_id: session_id.clone(),
        email,
        paid_articles,
        created_at: now,
        expires_at: now + (SESSION_EXPIRY_DAYS * NANOS_PER_DAY),
    };

    storage::insert_user_session(session.clone());
    Ok(session)
}

/// Validate a session and check expiration
pub fn validate_session(session_id: &str) -> AuthResult<UserSession> {
    let session = storage::get_user_session(session_id)
        .ok_or(AuthError::SessionNotFound)?;

    let now = ic_cdk::api::time();
    if now > session.expires_at {
        return Err(AuthError::SessionExpired);
    }

    Ok(session)
}

/// Add a paid article to an existing session
pub fn add_article_to_session(session_id: &str, article_slug: String) -> AuthResult<UserSession> {
    let mut session = validate_session(session_id)?;

    if !session.paid_articles.contains(&article_slug) {
        session.paid_articles.push(article_slug);
        storage::update_user_session(session.clone());
    }

    Ok(session)
}

/// Check if session grants access to a specific article
pub fn session_grants_access(session_id: &str, article_slug: &str) -> AuthResult<bool> {
    let session = validate_session(session_id)?;
    Ok(session.paid_articles.contains(&article_slug.to_string()))
}

// ============================================================================
// Gift Token Operations
// ============================================================================

/// Create a new gift token for sharing an article
pub async fn create_gift_token(
    article_slug: String,
    gifter_email: String,
    recipient_email: Option<String>,
) -> AuthResult<GiftToken> {
    let gift_token = generate_secure_token().await?;
    let now = ic_cdk::api::time();

    let gift = GiftToken {
        gift_token: gift_token.clone(),
        article_slug,
        gifter_email,
        recipient_email,
        redeemed: false,
        redeemed_by: None,
        created_at: now,
        expires_at: now + (GIFT_EXPIRY_DAYS * NANOS_PER_DAY),
    };

    storage::insert_gift_token(gift.clone());
    Ok(gift)
}

/// Redeem a gift token
pub fn redeem_gift_token(gift_token_id: &str, redeemer_email: String) -> AuthResult<GiftToken> {
    let mut gift = storage::get_gift_token(gift_token_id)
        .ok_or(AuthError::GiftNotFound)?;

    // Check if already redeemed
    if gift.redeemed {
        return Err(AuthError::GiftAlreadyRedeemed);
    }

    // Check expiration
    let now = ic_cdk::api::time();
    if now > gift.expires_at {
        return Err(AuthError::GiftExpired);
    }

    // Mark as redeemed
    gift.redeemed = true;
    gift.redeemed_by = Some(redeemer_email);
    storage::update_gift_token(gift.clone());

    Ok(gift)
}

// ============================================================================
// Unified Access Check
// ============================================================================

/// Check if user has access to an article via any method (token, session, or gift)
pub fn check_article_access(
    article_slug: &str,
    access_token: Option<&str>,
    session_id: Option<&str>,
) -> bool {
    // Check access token
    if let Some(token) = access_token {
        if token_grants_access(token, article_slug).unwrap_or(false) {
            return true;
        }
    }

    // Check session
    if let Some(session) = session_id {
        if session_grants_access(session, article_slug).unwrap_or(false) {
            return true;
        }
    }

    false
}

// ============================================================================
// HMAC Signing for Cookies
// ============================================================================

/// Sign data with HMAC-SHA256
pub fn sign_data(data: &str) -> String {
    HMAC_SECRET.with(|secret| {
        let mut mac = HmacSha256::new_from_slice(&secret.borrow())
            .expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    })
}

/// Verify HMAC signature
pub fn verify_signature(data: &str, signature: &str) -> bool {
    let expected = sign_data(data);
    // Constant-time comparison to prevent timing attacks
    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

/// Create a signed session cookie value
pub fn create_signed_cookie(session_id: &str) -> String {
    let signature = sign_data(session_id);
    format!("{}.{}", session_id, signature)
}

/// Verify and extract session ID from signed cookie
pub fn verify_signed_cookie(cookie: &str) -> AuthResult<String> {
    let parts: Vec<&str> = cookie.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return Err(AuthError::InvalidSignature);
    }

    let signature = parts[0];
    let session_id = parts[1];

    if !verify_signature(session_id, signature) {
        return Err(AuthError::InvalidSignature);
    }

    Ok(session_id.to_string())
}

/// Set the HMAC secret (should be called during canister initialization)
pub fn set_hmac_secret(secret: Vec<u8>) {
    HMAC_SECRET.with(|s| {
        *s.borrow_mut() = secret;
    });
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Constant-time byte comparison to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
