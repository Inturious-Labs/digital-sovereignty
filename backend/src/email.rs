//! Email sending via HTTP outcalls
//!
//! Handles:
//! - Sending access link emails after payment
//! - Sending gift notification emails
//! - Email template rendering

use candid::{CandidType, Deserialize};
use ic_cdk::management_canister::{
    http_request, transform_context_from_query, HttpHeader, HttpMethod, HttpRequestArgs,
    HttpRequestResult, TransformArgs,
};
use serde::Serialize;
use std::cell::RefCell;

// Configuration storage
thread_local! {
    static EMAIL_API_KEY: RefCell<String> = RefCell::new(String::new());
    static EMAIL_API_URL: RefCell<String> = RefCell::new("https://api.resend.com/emails".to_string());
    static SENDER_EMAIL: RefCell<String> = RefCell::new("hello@herbertyang.xyz".to_string());
    static SENDER_NAME: RefCell<String> = RefCell::new("Herbert Yang".to_string());
    static SITE_URL: RefCell<String> = RefCell::new("https://digitalsovereignty.herbertyang.xyz".to_string());
}

/// Result type for email operations
pub type EmailResult<T> = Result<T, EmailError>;

/// Email-related errors
#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum EmailError {
    NotConfigured,
    HttpRequestFailed(String),
    InvalidResponse(String),
    TemplateError(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::NotConfigured => write!(f, "Email service not configured"),
            EmailError::HttpRequestFailed(msg) => write!(f, "HTTP request failed: {}", msg),
            EmailError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            EmailError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Set email API key (e.g., Resend API key)
pub fn set_email_api_key(key: String) {
    EMAIL_API_KEY.with(|k| {
        *k.borrow_mut() = key;
    });
}

/// Set email API URL (defaults to Resend)
pub fn set_email_api_url(url: String) {
    EMAIL_API_URL.with(|u| {
        *u.borrow_mut() = url;
    });
}

/// Set sender email address
pub fn set_sender_email(email: String) {
    SENDER_EMAIL.with(|e| {
        *e.borrow_mut() = email;
    });
}

/// Set sender name
pub fn set_sender_name(name: String) {
    SENDER_NAME.with(|n| {
        *n.borrow_mut() = name;
    });
}

/// Set site URL for links
pub fn set_site_url(url: String) {
    SITE_URL.with(|u| {
        *u.borrow_mut() = url;
    });
}

/// Check if email service is configured
pub fn is_configured() -> bool {
    EMAIL_API_KEY.with(|k| !k.borrow().is_empty())
}

fn get_api_key() -> EmailResult<String> {
    EMAIL_API_KEY.with(|k| {
        let key = k.borrow().clone();
        if key.is_empty() {
            Err(EmailError::NotConfigured)
        } else {
            Ok(key)
        }
    })
}

fn get_api_url() -> String {
    EMAIL_API_URL.with(|u| u.borrow().clone())
}

fn get_sender() -> (String, String) {
    let email = SENDER_EMAIL.with(|e| e.borrow().clone());
    let name = SENDER_NAME.with(|n| n.borrow().clone());
    (email, name)
}

fn get_site_url() -> String {
    SITE_URL.with(|u| u.borrow().clone())
}

// ============================================================================
// Email Sending
// ============================================================================

/// Email request structure
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct SendEmailRequest {
    pub to: String,
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

/// Email response structure
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct SendEmailResponse {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Resend API request format
#[derive(Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

/// Resend API response format
#[derive(Deserialize)]
struct ResendEmailResponse {
    id: Option<String>,
    #[serde(default)]
    message: Option<String>,
}

/// Send an email via the configured email service
pub async fn send_email(request: SendEmailRequest) -> EmailResult<SendEmailResponse> {
    let api_key = get_api_key()?;
    let api_url = get_api_url();
    let (sender_email, sender_name) = get_sender();

    let resend_request = ResendEmailRequest {
        from: format!("{} <{}>", sender_name, sender_email),
        to: vec![request.to],
        subject: request.subject,
        html: request.html,
        text: request.text,
    };

    let body = serde_json::to_vec(&resend_request)
        .map_err(|e| EmailError::TemplateError(e.to_string()))?;

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", api_key),
        },
    ];

    let request_arg = HttpRequestArgs {
        url: api_url,
        method: HttpMethod::POST,
        body: Some(body),
        max_response_bytes: Some(5_000),
        transform: Some(transform_context_from_query(
            "transform_email_response".to_string(),
            vec![],
        )),
        headers: request_headers,
        is_replicated: Some(false), // Non-replicated outcall for deterministic response
    };

    let response: HttpRequestResult = http_request(&request_arg)
        .await
        .map_err(|e| EmailError::HttpRequestFailed(format!("{:?}", e)))?;

    if response.status >= 200u64 && response.status < 300u64 {
        let resend_response: ResendEmailResponse = serde_json::from_slice(&response.body)
            .unwrap_or(ResendEmailResponse {
                id: None,
                message: None,
            });

        Ok(SendEmailResponse {
            success: true,
            message_id: resend_response.id,
            error: None,
        })
    } else {
        let error_body = String::from_utf8_lossy(&response.body);
        Ok(SendEmailResponse {
            success: false,
            message_id: None,
            error: Some(format!("Status {}: {}", response.status, error_body)),
        })
    }
}

// ============================================================================
// Email Templates
// ============================================================================

/// Send payment confirmation email with access link
pub async fn send_access_email(
    to_email: &str,
    article_title: &str,
    article_slug: &str,
    access_token: &str,
) -> EmailResult<SendEmailResponse> {
    let site_url = get_site_url();
    let access_link = format!(
        "{}/posts/{}/?token={}",
        site_url, article_slug, access_token
    );

    let subject = format!("Your article is ready: {}", article_title);

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ margin-top: 40px; padding-top: 20px; border-top: 1px solid #eee; font-size: 14px; color: #666; }}
    </style>
</head>
<body>
    <h2>Thanks for your purchase!</h2>

    <p>You now have full access to:</p>

    <h3>"{}"</h3>

    <a href="{}" class="button">Read Article</a>

    <p>Or copy this link: <br><code>{}</code></p>

    <p>This link is yours to keep – bookmark it for future reference.</p>

    <div class="footer">
        <p>Questions? Just reply to this email.</p>
        <p>Best,<br>Herbert Yang<br>Digital Sovereignty Chronicle</p>
    </div>
</body>
</html>"#,
        article_title, access_link, access_link
    );

    let text = format!(
        r#"Thanks for your purchase!

You now have full access to: "{}"

Read your article: {}

This link is yours to keep – bookmark it for future reference.

Questions? Just reply to this email.

Best,
Herbert Yang
Digital Sovereignty Chronicle"#,
        article_title, access_link
    );

    send_email(SendEmailRequest {
        to: to_email.to_string(),
        subject,
        html,
        text: Some(text),
    })
    .await
}

/// Send gift notification email
pub async fn send_gift_email(
    to_email: &str,
    gifter_name: &str,
    gifter_email: &str,
    article_title: &str,
    article_slug: &str,
    gift_token: &str,
) -> EmailResult<SendEmailResponse> {
    let site_url = get_site_url();
    let gift_link = format!(
        "{}/posts/{}/?gift={}",
        site_url, article_slug, gift_token
    );

    let subject = format!("{} shared an article with you!", gifter_name);

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .gift-box {{ background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 20px; margin: 20px 0; }}
        .footer {{ margin-top: 40px; padding-top: 20px; border-top: 1px solid #eee; font-size: 14px; color: #666; }}
    </style>
</head>
<body>
    <h2>You've received a gift! 🎁</h2>

    <div class="gift-box">
        <p><strong>{}</strong> ({}) thought you'd enjoy this article from Digital Sovereignty Chronicle:</p>
        <h3>"{}"</h3>
        <p>This is a $5 article – yours free as a gift!</p>
    </div>

    <a href="{}" class="button">Read Article</a>

    <p>Or copy this link: <br><code>{}</code></p>

    <div class="footer">
        <p>Enjoy the read!</p>
        <p>Digital Sovereignty Chronicle</p>
    </div>
</body>
</html>"#,
        gifter_name, gifter_email, article_title, gift_link, gift_link
    );

    let text = format!(
        r#"You've received a gift!

{} ({}) thought you'd enjoy this article from Digital Sovereignty Chronicle:

"{}"

This is a $5 article – yours free as a gift!

Read your article: {}

Enjoy the read!

Digital Sovereignty Chronicle"#,
        gifter_name, gifter_email, article_title, gift_link
    );

    send_email(SendEmailRequest {
        to: to_email.to_string(),
        subject,
        html,
        text: Some(text),
    })
    .await
}

// ============================================================================
// Transform Function (for IC HTTP outcalls)
// ============================================================================

/// Transform function to clean up HTTP response for consensus
#[ic_cdk::query]
pub fn transform_email_response(args: TransformArgs) -> HttpRequestResult {
    let mut response = args.response;
    // Remove headers that may vary between replicas
    response.headers.clear();
    response
}

// ============================================================================
// Test Helpers
// ============================================================================

/// Email configuration status
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EmailConfigStatus {
    pub is_configured: bool,
    pub has_api_key: bool,
    pub sender_email: String,
    pub sender_name: String,
    pub site_url: String,
}

/// Get configuration status (for testing)
pub fn get_config_status() -> EmailConfigStatus {
    let (sender_email, sender_name) = get_sender();
    EmailConfigStatus {
        is_configured: is_configured(),
        has_api_key: EMAIL_API_KEY.with(|k| !k.borrow().is_empty()),
        sender_email,
        sender_name,
        site_url: get_site_url(),
    }
}

/// Generate preview of access email (for testing without sending)
pub fn preview_access_email(
    to_email: &str,
    article_title: &str,
    article_slug: &str,
    access_token: &str,
) -> SendEmailRequest {
    let site_url = get_site_url();
    let access_link = format!(
        "{}/posts/{}/?token={}",
        site_url, article_slug, access_token
    );

    SendEmailRequest {
        to: to_email.to_string(),
        subject: format!("Your article is ready: {}", article_title),
        html: format!(
            "<p>Access your article: <a href=\"{}\">{}</a></p>",
            access_link, article_title
        ),
        text: Some(format!("Access your article: {}", access_link)),
    }
}

/// Generate preview of gift email (for testing without sending)
pub fn preview_gift_email(
    to_email: &str,
    gifter_name: &str,
    article_title: &str,
    article_slug: &str,
    gift_token: &str,
) -> SendEmailRequest {
    let site_url = get_site_url();
    let gift_link = format!(
        "{}/posts/{}/?gift={}",
        site_url, article_slug, gift_token
    );

    SendEmailRequest {
        to: to_email.to_string(),
        subject: format!("{} shared an article with you!", gifter_name),
        html: format!(
            "<p>{} shared: <a href=\"{}\">{}</a></p>",
            gifter_name, gift_link, article_title
        ),
        text: Some(format!("{} shared: {}", gifter_name, gift_link)),
    }
}
