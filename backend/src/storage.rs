//! Persistent storage for tokens, sessions, and gifts
//!
//! Uses IC stable structures for data that survives canister upgrades.

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;

// Memory IDs for different storage regions
const ACCESS_TOKENS_MEM_ID: MemoryId = MemoryId::new(0);
const USER_SESSIONS_MEM_ID: MemoryId = MemoryId::new(1);
const GIFT_TOKENS_MEM_ID: MemoryId = MemoryId::new(2);

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Maximum sizes for storable types (in bytes)
const MAX_TOKEN_SIZE: u32 = 512;
const MAX_SESSION_SIZE: u32 = 2048; // Larger due to Vec<String>
const MAX_GIFT_SIZE: u32 = 512;
const MAX_KEY_SIZE: u32 = 128;

/// Access token issued after successful payment
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AccessToken {
    pub token: String,
    pub email: String,
    pub article_slug: String,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Storable for AccessToken {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_TOKEN_SIZE,
        is_fixed_size: false,
    };
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

impl Storable for UserSession {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_SESSION_SIZE,
        is_fixed_size: false,
    };
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

impl Storable for GiftToken {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_GIFT_SIZE,
        is_fixed_size: false,
    };
}

/// Wrapper for String keys to implement Storable
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StorableString(pub String);

impl Storable for StorableString {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        StorableString(Decode!(bytes.as_ref(), String).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}

// Thread-local storage for stable structures
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACCESS_TOKENS: RefCell<StableBTreeMap<StorableString, AccessToken, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ACCESS_TOKENS_MEM_ID))
        ));

    static USER_SESSIONS: RefCell<StableBTreeMap<StorableString, UserSession, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USER_SESSIONS_MEM_ID))
        ));

    static GIFT_TOKENS: RefCell<StableBTreeMap<StorableString, GiftToken, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(GIFT_TOKENS_MEM_ID))
        ));
}

// ============================================================================
// Access Token Operations
// ============================================================================

/// Store a new access token
pub fn insert_access_token(token: AccessToken) {
    let key = StorableString(token.token.clone());
    ACCESS_TOKENS.with(|tokens| {
        tokens.borrow_mut().insert(key, token);
    });
}

/// Retrieve an access token by its ID
pub fn get_access_token(token_id: &str) -> Option<AccessToken> {
    let key = StorableString(token_id.to_string());
    ACCESS_TOKENS.with(|tokens| tokens.borrow().get(&key))
}

/// Remove an access token
pub fn remove_access_token(token_id: &str) -> Option<AccessToken> {
    let key = StorableString(token_id.to_string());
    ACCESS_TOKENS.with(|tokens| tokens.borrow_mut().remove(&key))
}

/// Get all access tokens (for admin/debugging)
pub fn get_all_access_tokens() -> Vec<AccessToken> {
    ACCESS_TOKENS.with(|tokens| {
        tokens
            .borrow()
            .iter()
            .map(|(_, token)| token)
            .collect()
    })
}

/// Count total access tokens
pub fn count_access_tokens() -> u64 {
    ACCESS_TOKENS.with(|tokens| tokens.borrow().len())
}

// ============================================================================
// User Session Operations
// ============================================================================

/// Store a new user session
pub fn insert_user_session(session: UserSession) {
    let key = StorableString(session.session_id.clone());
    USER_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(key, session);
    });
}

/// Retrieve a user session by its ID
pub fn get_user_session(session_id: &str) -> Option<UserSession> {
    let key = StorableString(session_id.to_string());
    USER_SESSIONS.with(|sessions| sessions.borrow().get(&key))
}

/// Update a user session (e.g., add new paid article)
pub fn update_user_session(session: UserSession) {
    insert_user_session(session);
}

/// Remove a user session
pub fn remove_user_session(session_id: &str) -> Option<UserSession> {
    let key = StorableString(session_id.to_string());
    USER_SESSIONS.with(|sessions| sessions.borrow_mut().remove(&key))
}

/// Count total user sessions
pub fn count_user_sessions() -> u64 {
    USER_SESSIONS.with(|sessions| sessions.borrow().len())
}

// ============================================================================
// Gift Token Operations
// ============================================================================

/// Store a new gift token
pub fn insert_gift_token(gift: GiftToken) {
    let key = StorableString(gift.gift_token.clone());
    GIFT_TOKENS.with(|gifts| {
        gifts.borrow_mut().insert(key, gift);
    });
}

/// Retrieve a gift token by its ID
pub fn get_gift_token(gift_token_id: &str) -> Option<GiftToken> {
    let key = StorableString(gift_token_id.to_string());
    GIFT_TOKENS.with(|gifts| gifts.borrow().get(&key))
}

/// Update a gift token (e.g., mark as redeemed)
pub fn update_gift_token(gift: GiftToken) {
    insert_gift_token(gift);
}

/// Remove a gift token
pub fn remove_gift_token(gift_token_id: &str) -> Option<GiftToken> {
    let key = StorableString(gift_token_id.to_string());
    GIFT_TOKENS.with(|gifts| gifts.borrow_mut().remove(&key))
}

/// Count total gift tokens
pub fn count_gift_tokens() -> u64 {
    GIFT_TOKENS.with(|gifts| gifts.borrow().len())
}
