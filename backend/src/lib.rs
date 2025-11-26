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
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

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

// ============================================================================
// Stripe test endpoints
// ============================================================================

/// Configure Stripe API keys (for testing)
#[update]
fn stripe_configure(secret_key: String, webhook_secret: String) {
    stripe::set_stripe_secret_key(secret_key);
    stripe::set_stripe_webhook_secret(webhook_secret);
}

/// Get Stripe configuration status
#[query]
fn stripe_status() -> stripe::StripeConfigStatus {
    stripe::get_config_status()
}

/// Test webhook signature verification
#[query]
fn stripe_test_webhook_verify(payload: String, signature: String) -> StripeWebhookTestResponse {
    // Use a large tolerance for testing (1 hour)
    match stripe::verify_webhook_signature(payload.as_bytes(), &signature, 3600) {
        Ok(()) => StripeWebhookTestResponse {
            valid: true,
            error: None,
        },
        Err(e) => StripeWebhookTestResponse {
            valid: false,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct StripeWebhookTestResponse {
    pub valid: bool,
    pub error: Option<String>,
}

/// Test webhook event parsing
#[query]
fn stripe_test_parse_event(payload: String) -> StripeParseEventResponse {
    match stripe::parse_webhook_event(payload.as_bytes()) {
        Ok(event) => StripeParseEventResponse {
            success: true,
            event: Some(event),
            error: None,
        },
        Err(e) => StripeParseEventResponse {
            success: false,
            event: None,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct StripeParseEventResponse {
    pub success: bool,
    pub event: Option<stripe::WebhookEvent>,
    pub error: Option<String>,
}

// ============================================================================
// Email test endpoints
// ============================================================================

/// Configure email service
#[update]
fn email_configure(api_key: String) {
    email::set_email_api_key(api_key);
}

/// Get email configuration status
#[query]
fn email_status() -> email::EmailConfigStatus {
    email::get_config_status()
}

/// Preview access email (without sending)
#[query]
fn email_preview_access(
    to_email: String,
    article_title: String,
    article_slug: String,
    access_token: String,
) -> email::SendEmailRequest {
    email::preview_access_email(&to_email, &article_title, &article_slug, &access_token)
}

/// Preview gift email (without sending)
#[query]
fn email_preview_gift(
    to_email: String,
    gifter_name: String,
    article_title: String,
    article_slug: String,
    gift_token: String,
) -> email::SendEmailRequest {
    email::preview_gift_email(&to_email, &gifter_name, &article_title, &article_slug, &gift_token)
}

// ============================================================================
// Production Endpoints
// ============================================================================

/// Create a payment intent for article purchase
/// Called by frontend when user clicks "Unlock Article ($5)"
#[update]
async fn create_payment(request: CreatePaymentRequest) -> CreatePaymentResponse {
    let stripe_request = stripe::CreatePaymentRequest {
        email: request.email,
        article_slug: request.article_slug,
        article_title: request.article_title,
    };

    match stripe::create_payment_intent(stripe_request).await {
        Ok(response) => CreatePaymentResponse {
            success: true,
            payment_intent_id: Some(response.payment_intent_id),
            client_secret: Some(response.client_secret),
            error: None,
        },
        Err(e) => CreatePaymentResponse {
            success: false,
            payment_intent_id: None,
            client_secret: None,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct CreatePaymentRequest {
    pub email: String,
    pub article_slug: String,
    pub article_title: String,
}

#[derive(CandidType, Deserialize)]
pub struct CreatePaymentResponse {
    pub success: bool,
    pub payment_intent_id: Option<String>,
    pub client_secret: Option<String>,
    pub error: Option<String>,
}

/// Handle Stripe webhook for payment completion
/// Called by Stripe when payment succeeds
#[update]
async fn handle_webhook(payload: String, signature: String) -> WebhookResponse {
    // Verify webhook signature (5 minute tolerance)
    if let Err(e) = stripe::verify_webhook_signature(payload.as_bytes(), &signature, 300) {
        return WebhookResponse {
            success: false,
            message: format!("Signature verification failed: {}", e),
        };
    }

    // Parse the event
    let event = match stripe::parse_webhook_event(payload.as_bytes()) {
        Ok(e) => e,
        Err(e) => {
            return WebhookResponse {
                success: false,
                message: format!("Failed to parse event: {}", e),
            };
        }
    };

    // Only process successful payments
    if !stripe::is_payment_succeeded(&event) {
        return WebhookResponse {
            success: true,
            message: format!("Ignored event type: {}", event.event_type),
        };
    }

    // Extract required fields
    let email = match event.email {
        Some(e) => e,
        None => {
            return WebhookResponse {
                success: false,
                message: "Missing email in payment metadata".to_string(),
            };
        }
    };

    let article_slug = match event.article_slug {
        Some(s) => s,
        None => {
            return WebhookResponse {
                success: false,
                message: "Missing article_slug in payment metadata".to_string(),
            };
        }
    };

    let article_title = event.article_title.unwrap_or_else(|| article_slug.clone());

    // Create access token
    let access_token = match auth::create_access_token(email.clone(), article_slug.clone()).await {
        Ok(token) => token,
        Err(e) => {
            return WebhookResponse {
                success: false,
                message: format!("Failed to create access token: {}", e),
            };
        }
    };

    // Send confirmation email
    if let Err(e) = email::send_access_email(&email, &article_title, &article_slug, &access_token.token).await {
        // Log error but don't fail - token was created successfully
        return WebhookResponse {
            success: true,
            message: format!("Token created but email failed: {}", e),
        };
    }

    WebhookResponse {
        success: true,
        message: format!("Payment processed, token created, email sent to {}", email),
    }
}

#[derive(CandidType, Deserialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message: String,
}

/// Validate access to an article
/// Called by frontend to check if user can view full content
#[query]
fn validate_access(request: ValidateAccessRequest) -> ValidateAccessResponse {
    let has_access = auth::check_article_access(
        &request.article_slug,
        request.access_token.as_deref(),
        request.session_id.as_deref(),
    );

    ValidateAccessResponse {
        has_access,
        article_slug: request.article_slug,
    }
}

#[derive(CandidType, Deserialize)]
pub struct ValidateAccessRequest {
    pub article_slug: String,
    pub access_token: Option<String>,
    pub session_id: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct ValidateAccessResponse {
    pub has_access: bool,
    pub article_slug: String,
}

/// Create a gift token for sharing an article
/// Called when a paid user wants to gift an article to someone
#[update]
async fn create_gift(request: CreateGiftRequest) -> CreateGiftResponse {
    // Verify the gifter has access to this article
    if !auth::check_article_access(&request.article_slug, Some(&request.gifter_token), None) {
        return CreateGiftResponse {
            success: false,
            gift_token: None,
            gift_url: None,
            error: Some("You don't have access to this article".to_string()),
        };
    }

    // Get gifter's email from their token
    let gifter_email = match auth::validate_access_token(&request.gifter_token) {
        Ok(token) => token.email,
        Err(e) => {
            return CreateGiftResponse {
                success: false,
                gift_token: None,
                gift_url: None,
                error: Some(format!("Invalid gifter token: {}", e)),
            };
        }
    };

    // Create the gift token
    let gift = match auth::create_gift_token(
        request.article_slug.clone(),
        gifter_email.clone(),
        request.recipient_email.clone(),
    ).await {
        Ok(g) => g,
        Err(e) => {
            return CreateGiftResponse {
                success: false,
                gift_token: None,
                gift_url: None,
                error: Some(format!("Failed to create gift: {}", e)),
            };
        }
    };

    // Build gift URL
    let site_url = "https://digitalsovereignty.herbertyang.xyz";
    let gift_url = format!("{}/posts/{}/?gift={}", site_url, request.article_slug, gift.gift_token);

    // If recipient email provided, send gift notification
    if let Some(ref recipient) = request.recipient_email {
        let gifter_name = request.gifter_name.unwrap_or_else(|| gifter_email.clone());
        let article_title = request.article_title.unwrap_or_else(|| request.article_slug.clone());

        let _ = email::send_gift_email(
            recipient,
            &gifter_name,
            &gifter_email,
            &article_title,
            &request.article_slug,
            &gift.gift_token,
        ).await;
    }

    CreateGiftResponse {
        success: true,
        gift_token: Some(gift.gift_token),
        gift_url: Some(gift_url),
        error: None,
    }
}

#[derive(CandidType, Deserialize)]
pub struct CreateGiftRequest {
    pub article_slug: String,
    pub gifter_token: String,
    pub gifter_name: Option<String>,
    pub article_title: Option<String>,
    pub recipient_email: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateGiftResponse {
    pub success: bool,
    pub gift_token: Option<String>,
    pub gift_url: Option<String>,
    pub error: Option<String>,
}

/// Redeem a gift token to get article access
/// Called when someone clicks a gift link
#[update]
fn redeem_gift(gift_token: String, redeemer_email: String) -> RedeemGiftResponse {
    match auth::redeem_gift_token(&gift_token, redeemer_email.clone()) {
        Ok(gift) => RedeemGiftResponse {
            success: true,
            article_slug: Some(gift.article_slug),
            error: None,
        },
        Err(e) => RedeemGiftResponse {
            success: false,
            article_slug: None,
            error: Some(e.to_string()),
        },
    }
}

#[derive(CandidType, Deserialize)]
pub struct RedeemGiftResponse {
    pub success: bool,
    pub article_slug: Option<String>,
    pub error: Option<String>,
}

/// Create a session for a user after validating their access token
/// Returns a signed session cookie for persistent access
#[update]
async fn create_session(access_token: String) -> CreateSessionResponse {
    // Validate the access token first
    let token_data = match auth::validate_access_token(&access_token) {
        Ok(t) => t,
        Err(e) => {
            return CreateSessionResponse {
                success: false,
                session_cookie: None,
                error: Some(format!("Invalid access token: {}", e)),
            };
        }
    };

    // Create a session with the article from the token
    let session = match auth::create_session(
        token_data.email,
        Some(token_data.article_slug),
    ).await {
        Ok(s) => s,
        Err(e) => {
            return CreateSessionResponse {
                success: false,
                session_cookie: None,
                error: Some(format!("Failed to create session: {}", e)),
            };
        }
    };

    // Create a signed cookie for the session
    let signed_cookie = auth::create_signed_cookie(&session.session_id);

    CreateSessionResponse {
        success: true,
        session_cookie: Some(signed_cookie),
        error: None,
    }
}

#[derive(CandidType, Deserialize)]
pub struct CreateSessionResponse {
    pub success: bool,
    pub session_cookie: Option<String>,
    pub error: Option<String>,
}

// Export Candid interface
ic_cdk::export_candid!();
