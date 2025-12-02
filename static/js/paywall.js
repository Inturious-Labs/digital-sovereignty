/**
 * Paywall Frontend Logic
 *
 * Handles:
 * - Access token validation from URL params or cookies
 * - Unlocking content for paid users
 * - Session management via cookies
 */

(function() {
  'use strict';

  // Configuration
  const CONFIG = {
    // Backend canister URL (IC mainnet)
    backendUrl: 'https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io',
    // Cookie name for session
    sessionCookieName: 'ds_session',
    // Cookie name for access tokens (per article)
    accessCookiePrefix: 'ds_access_',
    // Cookie expiry in days
    cookieExpiryDays: 365
  };

  // DOM Elements
  let paywallContainer = null;
  let paywallPrompt = null;
  let fullContent = null;
  let giftSection = null;
  let unlockButton = null;

  /**
   * Initialize paywall on page load
   */
  function init() {
    // Find paywall container
    paywallContainer = document.querySelector('.paywall-container');

    // Exit if not a paywalled article
    if (!paywallContainer) {
      return;
    }

    // Cache DOM elements
    paywallPrompt = document.getElementById('paywall-prompt');
    fullContent = document.getElementById('paywall-full-content');
    giftSection = document.getElementById('paywall-gift-section');
    unlockButton = document.getElementById('paywall-unlock-btn');

    // Get article info from data attributes
    const articleSlug = paywallContainer.dataset.articleSlug;
    const price = paywallContainer.dataset.price;

    console.log('[Paywall] Initializing for article:', articleSlug);

    // Check for access token in URL
    const urlParams = new URLSearchParams(window.location.search);
    const urlToken = urlParams.get('token');
    const giftToken = urlParams.get('gift');

    // Check for stored access in cookie
    const storedToken = getCookie(CONFIG.accessCookiePrefix + articleSlug);
    const sessionId = getCookie(CONFIG.sessionCookieName);

    // Determine which token to use
    const accessToken = urlToken || storedToken;

    if (giftToken) {
      // Handle gift redemption
      handleGiftRedemption(giftToken, articleSlug);
    } else if (accessToken || sessionId) {
      // Validate access
      validateAccess(articleSlug, accessToken, sessionId);
    } else {
      console.log('[Paywall] No access token found, showing paywall');
    }

    // Set up unlock button click handler
    if (unlockButton) {
      unlockButton.addEventListener('click', function() {
        handleUnlockClick(articleSlug, price);
      });
    }
  }

  /**
   * Validate access with backend
   */
  async function validateAccess(articleSlug, accessToken, sessionId) {
    console.log('[Paywall] Validating access...');

    try {
      const response = await fetch(`${CONFIG.backendUrl}/validate-access`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          article_slug: articleSlug,
          access_token: accessToken || null,
          session_id: sessionId || null
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();

      if (result.has_access) {
        console.log('[Paywall] Access granted');

        // Store token in cookie if from URL
        if (accessToken && !getCookie(CONFIG.accessCookiePrefix + articleSlug)) {
          setCookie(CONFIG.accessCookiePrefix + articleSlug, accessToken, CONFIG.cookieExpiryDays);
        }

        // Clean URL if token was in params
        if (window.location.search.includes('token=')) {
          cleanUrl();
        }

        unlockContent();
      } else {
        console.log('[Paywall] Access denied');
      }
    } catch (error) {
      console.error('[Paywall] Validation error:', error);
      // On error, keep paywall visible (fail secure)
    }
  }

  /**
   * Handle gift token redemption
   */
  async function handleGiftRedemption(giftToken, articleSlug) {
    console.log('[Paywall] Redeeming gift token...');

    // For gift redemption, we need user's email
    // For now, prompt for email (can be improved with modal later)
    const email = prompt('Enter your email to redeem this gift:');

    if (!email) {
      console.log('[Paywall] Gift redemption cancelled');
      return;
    }

    try {
      const response = await fetch(`${CONFIG.backendUrl}/redeem-gift`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          gift_token: giftToken,
          redeemer_email: email
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();

      if (result.success) {
        console.log('[Paywall] Gift redeemed successfully');

        // Store access and clean URL
        setCookie(CONFIG.accessCookiePrefix + articleSlug, giftToken, CONFIG.cookieExpiryDays);
        cleanUrl();
        unlockContent();

        alert('Gift redeemed! You now have full access to this article.');
      } else {
        console.error('[Paywall] Gift redemption failed:', result.error);
        alert('Unable to redeem gift: ' + (result.error || 'Unknown error'));
      }
    } catch (error) {
      console.error('[Paywall] Gift redemption error:', error);
      alert('Error redeeming gift. Please try again.');
    }
  }

  /**
   * Handle unlock button click
   * This will be expanded in Step 6 to integrate Stripe
   */
  function handleUnlockClick(articleSlug, price) {
    console.log('[Paywall] Unlock clicked for:', articleSlug, 'Price:', price);

    // TODO: Step 6 - Integrate Stripe checkout
    alert('Payment integration coming soon! Price: $' + price);
  }

  /**
   * Unlock content - show full article, hide paywall prompt
   */
  function unlockContent() {
    if (paywallPrompt) {
      paywallPrompt.style.display = 'none';
    }

    if (fullContent) {
      fullContent.classList.add('unlocked');
    }

    if (giftSection) {
      giftSection.classList.add('visible');
    }

    // Hide the preview fade effect
    const preview = document.querySelector('.paywall-preview');
    if (preview) {
      preview.style.display = 'none';
    }
  }

  /**
   * Remove token params from URL without reload
   */
  function cleanUrl() {
    const url = new URL(window.location);
    url.searchParams.delete('token');
    url.searchParams.delete('gift');
    window.history.replaceState({}, '', url);
  }

  // ============================================================================
  // Cookie Utilities
  // ============================================================================

  function setCookie(name, value, days) {
    const expires = new Date(Date.now() + days * 864e5).toUTCString();
    document.cookie = `${name}=${encodeURIComponent(value)}; expires=${expires}; path=/; SameSite=Lax`;
  }

  function getCookie(name) {
    const cookies = document.cookie.split('; ');
    for (const cookie of cookies) {
      const [key, val] = cookie.split('=');
      if (key === name) {
        return decodeURIComponent(val);
      }
    }
    return null;
  }

  function deleteCookie(name) {
    document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/`;
  }

  // ============================================================================
  // Initialize on DOM ready
  // ============================================================================

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

})();
