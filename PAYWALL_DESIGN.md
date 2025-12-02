# Digital Sovereignty Chronicle - Paywall System Design

## Overview

This document outlines the design and implementation plan for adding a pay-per-article paywall feature to the Digital Sovereignty Chronicle newsletter. The system enables a flexible revenue model where readers pay $5 per article instead of monthly subscriptions.

## Current Architecture

- **Frontend**: Hugo static site with "paper" theme
- **Hosting**: Internet Computer canister `wupbw-2aaaa-aaaae-abn7a-cai`
- **Domain**: digitalsovereignty.herbertyang.xyz
- **Email**: Buttondown integrated with Stripe for subscriptions
- **Content**: Markdown files in `/content/posts/YYYY/MM/DD-slug/index.md`

## Requirements

### Core Features
1. **Pay-per-article**: $5 per article payment model
2. **Preview access**: Free users see title + preview, paid users see full content
3. **Private links**: Paid users receive unique access links via email
4. **Persistent access**: Private links work indefinitely, website remembers paid access
5. **Gift sharing**: Paid users can share articles with friends as gifts
6. **SEO optimization**: Limited content exposure to bots for indexing
7. **Security**: Protect sensitive content from free crawling/scraping

### User Experience
- **Free users**: See preview + "Unlock for $5" button
- **Paid users**: Full article access + gift sharing options
- **Gift recipients**: Full access but cannot re-gift
- **Email integration**: Automatic delivery of access links post-payment

## System Architecture

### Technology Decisions

**Backend**: Rust canister (Internet Computer)
- **Why Rust**: Complex Stripe integration, cryptographic security, robust error handling
- **Why not Motoko**: Limited ecosystem for payment processing and crypto operations

**Frontend**: Hugo with JavaScript enhancements
- **Approach**: Server-side rendering + client-side paywall logic
- **SEO Strategy**: Limited bot content exposure (title + description + preview only)

### Components

#### 1. Backend Canister (Rust)
```
paywall-backend/
├── src/
│   ├── main.rs           # HTTP handlers & routing
│   ├── stripe.rs         # Stripe Payment Intent & webhook handling
│   ├── auth.rs           # Token generation & session management
│   ├── storage.rs        # Data persistence (tokens, sessions, gifts)
│   └── email.rs          # Email sending via HTTP outcalls
├── Cargo.toml
└── dfx.json
```

**Key Endpoints:**
- `POST /create-checkout-session` - Create Stripe Checkout Session, returns checkout URL
- `POST /verify-payment` - Verify completed payment, returns access token (for immediate unlock)
- `POST /stripe-webhook` - Handle payment completion notifications (backup/async flow)
- `POST /validate-access` - Check if user has access to article
- `POST /create-gift` - Generate gift access tokens
- `POST /redeem-gift` - Redeem gift access
- `POST /create-session` - Create user session from access token

#### 2. Frontend Modifications (Hugo)

**New Front Matter Schema:**
```yaml
---
title: "Article Title"
paywall: true
price: 5
preview: "Preview text shown to free users and bots..."
description: "SEO meta description"
---
```

**Template Updates:**
- Article layout detects `paywall: true`
- Shows preview + unlock button for unpaid users
- Shows full content for paid users (via JavaScript)
- Includes gift sharing UI for paid users
- Bot detection for SEO (limited content exposure)

**New Static Assets:**
- `static/js/paywall.js` - Client-side paywall logic
- `static/css/paywall.css` - Paywall styling

## Data Models

### Access Tokens
```rust
struct AccessToken {
    token: String,        // 32-byte cryptographically secure hex
    email: String,        // Purchaser email
    article_slug: String, // Article identifier
    expires_at: u64,      // Unix timestamp (1 year from creation)
    created_at: u64,      // Purchase timestamp
}
```

### User Sessions
```rust
struct UserSession {
    session_id: String,      // Session identifier cookie
    email: String,           // User email
    paid_articles: Vec<String>, // List of article slugs user has paid for
    expires_at: u64,         // Session expiration (30 days)
    created_at: u64,         // Session creation
}
```

### Gift Access
```rust
struct GiftToken {
    gift_token: String,   // Unique gift identifier
    article_slug: String, // Article being gifted
    gifter_email: String, // Person who paid and is gifting
    recipient_email: Option<String>, // Recipient (if sent via email)
    redeemed: bool,       // Whether gift has been used
    redeemed_by: Option<String>, // Who redeemed it
    expires_at: u64,      // Gift expiration
    created_at: u64,      // Gift creation timestamp
}
```

## Payment & Access Flow

### Purchase Flow (Immediate Unlock)

The purchase flow is designed for **immediate access** after payment - no email check required.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PURCHASE FLOW DIAGRAM                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   USER                    FRONTEND                BACKEND              STRIPE│
│    │                         │                       │                    │  │
│    │ 1. Click "Unlock $5"    │                       │                    │  │
│    │────────────────────────>│                       │                    │  │
│    │                         │                       │                    │  │
│    │                         │ 2. POST /create-checkout-session           │  │
│    │                         │──────────────────────>│                    │  │
│    │                         │                       │                    │  │
│    │                         │                       │ 3. Create Checkout │  │
│    │                         │                       │    Session         │  │
│    │                         │                       │───────────────────>│  │
│    │                         │                       │                    │  │
│    │                         │                       │<───────────────────│  │
│    │                         │                       │   session_id + url │  │
│    │                         │                       │                    │  │
│    │                         │<──────────────────────│                    │  │
│    │                         │   checkout_url        │                    │  │
│    │                         │                       │                    │  │
│    │<────────────────────────│                       │                    │  │
│    │   Redirect to Stripe    │                       │                    │  │
│    │                         │                       │                    │  │
│    │ 4. Enter email + pay    │                       │                    │  │
│    │─────────────────────────────────────────────────────────────────────>│  │
│    │                         │                       │                    │  │
│    │<─────────────────────────────────────────────────────────────────────│  │
│    │   Redirect back with ?session_id=xxx           │                    │  │
│    │                         │                       │                    │  │
│    │                         │ 5. POST /verify-payment (session_id)       │  │
│    │                         │──────────────────────>│                    │  │
│    │                         │                       │                    │  │
│    │                         │                       │ 6. Retrieve session│  │
│    │                         │                       │    from Stripe     │  │
│    │                         │                       │───────────────────>│  │
│    │                         │                       │<───────────────────│  │
│    │                         │                       │   email, payment   │  │
│    │                         │                       │                    │  │
│    │                         │                       │ 7. Create access   │  │
│    │                         │                       │    token, store,   │  │
│    │                         │                       │    send email      │  │
│    │                         │                       │                    │  │
│    │                         │<──────────────────────│                    │  │
│    │                         │   access_token        │                    │  │
│    │                         │                       │                    │  │
│    │                         │ 8. Store token in     │                    │  │
│    │                         │    cookie, unlock     │                    │  │
│    │                         │    content            │                    │  │
│    │                         │                       │                    │  │
│    │<────────────────────────│                       │                    │  │
│    │   ARTICLE UNLOCKED!     │                       │                    │  │
│    │                         │                       │                    │  │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Step-by-step:**

1. **User clicks "Unlock Article ($5)"** → Frontend JS initiates checkout
2. **Frontend calls `POST /create-checkout-session`** → Sends article_slug, article_title
3. **Backend creates Stripe Checkout Session** → Returns checkout URL with success redirect
4. **User redirected to Stripe** → Enters email and payment info on Stripe's hosted page
5. **Payment succeeds** → Stripe redirects user back to article with `?session_id=xxx`
6. **Frontend detects session_id** → Calls `POST /verify-payment` with session_id
7. **Backend verifies with Stripe** → Retrieves customer email from Stripe session
8. **Backend creates access token** → Stores in DB, returns token to frontend
9. **Frontend stores token in cookie** → Unlocks article **immediately**
10. **Backend sends confirmation email** → Private link for future access (async, non-blocking)

**Key Design Decisions:**

- **No email collection on our site**: Stripe Checkout collects email during payment
- **Immediate unlock**: User sees content right after payment, no email check required
- **Email sent in background**: Private link email is for convenience (future visits, other devices)
- **Webhook as backup**: Stripe webhook still processes payments (handles edge cases like browser close)

### Legacy Flow (Email-First) - Deprecated

The original design required users to check email for access. This is now deprecated in favor of immediate unlock above. The webhook flow remains as a backup mechanism.

### Gift Flow
1. **Paid user clicks "Gift this Article"** → Modal with email/link options
2. **Email option**: Enter friend's email → Backend generates gift token → Sends gift email
3. **Link option**: Get shareable gift URL for manual sharing
4. **Friend redeems gift** → Backend validates gift token → Grants access
5. **Recipient access**: Gets full article access but cannot re-gift

## Security Model

### Access Control
- **Cryptographically secure tokens**: 32-byte random generation using IC random beacon
- **Token expiration**: Access tokens valid for 1 year
- **Session management**: Browser sessions expire after 30 days
- **Gift limitations**: Recipients cannot create new gifts (prevents viral free access)

### Content Protection
- **No full content in HTML**: Bots only see title + description + preview
- **Client-side gating**: Full content loaded via JavaScript for verified users only
- **Private URLs**: Access tokens in query parameters for email links
- **Session persistence**: Reduces need for repeated private link access

## SEO Strategy

### Bot Content Exposure (Limited)
- **Title**: Full article title for search results
- **Meta description**: Optimized description for search snippets
- **Preview content**: First 150 words or custom preview text
- **Schema markup**: Article structured data for rich snippets
- **URL structure**: SEO-friendly slugs maintained

### Search Benefits
- **Discoverability**: Articles appear in search results with compelling previews
- **Click-through**: Preview creates interest without giving away full value
- **Indexing**: Proper meta tags and structured data for search engines
- **Social sharing**: Open Graph tags for social media previews

### Content Security
- **No full text exposure**: Sensitive content protected from LLM training scraping
- **Bot detection**: Server-side User-Agent filtering
- **JavaScript gating**: Full content only revealed to authenticated human users

## Implementation Phases

### Phase 1: Backend Canister Development (3-4 days)
1. **Project setup**: Create Rust canister project structure
2. **Stripe integration**: Payment Intent creation and webhook handling
3. **Authentication system**: Token generation, validation, session management
4. **Data storage**: Persistent storage for tokens, sessions, gifts
5. **Email system**: HTTP outcalls for sending access and gift emails
6. **API endpoints**: All REST endpoints for frontend integration

### Phase 2: Frontend Modifications (2-3 days)
1. **Template updates**: Hugo layouts for paywall detection and UI
2. **JavaScript integration**: Client-side paywall logic and access control
3. **Styling**: CSS for paywall UI components and user experience
4. **Gift interface**: UI for creating and sharing article gifts
5. **Testing**: Frontend integration with backend API

### Phase 3: Integration & Testing (1-2 days)
1. **Canister deployment**: Deploy backend to IC mainnet
2. **Stripe configuration**: Set up webhook endpoints and API keys
3. **Frontend deployment**: Update and redeploy Hugo site
4. **End-to-end testing**: Complete purchase and access flow testing
5. **Email testing**: Verify delivery of access links and gift notifications

## Email Templates

### Payment Confirmation Email
```
Subject: Your Digital Sovereignty Chronicle article is ready!

Hi there,

Thanks for your $5 purchase! You now have full access to:

"[Article Title]"

Access your article: [Private Link]

This link is yours to keep - bookmark it for future reference.

Questions? Just reply to this email.

Best,
Herbert Yang
```

### Gift Email Template
```
Subject: [Gifter Name] shared a $5 article gift with you!

Hi there,

[Gifter Name] ([gifter@email.com]) thought you'd enjoy this article from Digital Sovereignty Chronicle:

"[Article Title]"

This is a $5 gift - click to read the full article:
[Gift Access Link]

Enjoy the read!

Digital Sovereignty Chronicle
```

## Configuration Requirements

### Environment Variables
```bash
# Stripe
STRIPE_SECRET_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Email Service (via HTTP outcalls)
EMAIL_SERVICE_URL=https://api.your-email-service.com
EMAIL_API_KEY=...

# Security
HMAC_SECRET=... # For session signing
```

### IC Canister Configuration
- **Cycles**: Ensure sufficient cycles for HTTP outcalls (email sending)
- **Permissions**: Configure for HTTP outcalls and external API access
- **Storage**: Stable storage for persistent token/session data

## Monitoring & Analytics

### Key Metrics to Track
- **Conversion rate**: Preview views → purchases
- **Gift utilization**: Gift creation → redemption rate
- **Revenue tracking**: Total article sales, average revenue per article
- **Access patterns**: Token usage, session duration, return visits

### Error Monitoring
- **Payment failures**: Stripe webhook processing errors
- **Email delivery**: HTTP outcall failures for email sending
- **Token validation**: Invalid/expired token access attempts
- **Session management**: Cookie/session-related errors

## Future Enhancements

### Potential Features
- **Bulk purchase discounts**: Buy 5 articles for $20
- **Subscription hybrid**: Monthly fee + individual article credits
- **Author analytics**: Detailed metrics on article performance
- **Reader profiles**: User dashboard showing purchased articles
- **Mobile app**: Native mobile access with stored purchases

### Technical Improvements
- **Caching layer**: Redis-like caching for frequent access validation
- **CDN integration**: Faster content delivery for paid users
- **A/B testing**: Preview length and pricing optimization
- **Analytics dashboard**: Real-time revenue and usage metrics

---

*This design document serves as the implementation blueprint for the Digital Sovereignty Chronicle paywall system. All code and configuration will be tracked in the `feature/paywall-system` branch.*