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

    // Check for URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    const urlToken = urlParams.get('token');
    const giftToken = urlParams.get('gift');
    const stripeSessionId = urlParams.get('session_id'); // Returned from Stripe checkout

    // Check for stored access in cookie
    const storedToken = getCookie(CONFIG.accessCookiePrefix + articleSlug);
    const sessionId = getCookie(CONFIG.sessionCookieName);

    // Determine which token to use
    const accessToken = urlToken || storedToken;

    if (stripeSessionId) {
      // User just returned from Stripe checkout - verify payment and unlock
      handleStripeReturn(stripeSessionId, articleSlug);
    } else if (giftToken) {
      // Handle gift redemption
      handleGiftRedemption(giftToken, articleSlug);
    } else if (accessToken || sessionId) {
      // Validate existing access
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
   * Handle unlock button click - initiate Stripe Checkout
   */
  async function handleUnlockClick(articleSlug, price) {
    console.log('[Paywall] Unlock clicked for:', articleSlug, 'Price:', price);

    // Disable button to prevent double-clicks
    if (unlockButton) {
      unlockButton.disabled = true;
      unlockButton.textContent = 'Processing...';
    }

    try {
      // Get article title from page
      const articleTitle = document.querySelector('h1')?.textContent || articleSlug;

      // Current page URL for redirect
      const currentUrl = window.location.origin + window.location.pathname;

      // Call backend to create Stripe Checkout Session
      const response = await fetch(`${CONFIG.backendUrl}/create-checkout-session`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          article_slug: articleSlug,
          article_title: articleTitle,
          success_url: currentUrl,
          cancel_url: currentUrl
        })
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Server error (${response.status}): ${errorText.substring(0, 100)}`);
      }

      // Check if response is JSON
      const contentType = response.headers.get('content-type');
      if (!contentType || !contentType.includes('application/json')) {
        throw new Error('Payment service is not available. Please try again later.');
      }

      const result = await response.json();

      if (result.success && result.checkout_url) {
        console.log('[Paywall] Redirecting to Stripe checkout...');
        // Redirect to Stripe Checkout
        window.location.href = result.checkout_url;
      } else {
        throw new Error(result.error || 'Failed to create checkout session');
      }
    } catch (error) {
      console.error('[Paywall] Checkout error:', error);

      // User-friendly error message
      let userMessage = 'Unable to start checkout. Please try again.';
      if (error.message.includes('Failed to fetch') || error.message.includes('NetworkError')) {
        userMessage = 'Unable to connect to payment service. Please check your internet connection and try again.';
      } else if (error.message.includes('not available')) {
        userMessage = 'Payment service is temporarily unavailable. Please try again later.';
      } else {
        userMessage = `Checkout failed: ${error.message}`;
      }

      alert(userMessage);

      // Re-enable button
      if (unlockButton) {
        unlockButton.disabled = false;
        unlockButton.textContent = `Unlock for $${price}`;
      }
    }
  }

  /**
   * Handle return from Stripe Checkout
   * Verify payment and unlock content immediately
   */
  async function handleStripeReturn(stripeSessionId, articleSlug) {
    console.log('[Paywall] Verifying Stripe payment...');

    // Show loading state
    if (paywallPrompt) {
      paywallPrompt.innerHTML = `
        <div class="paywall-loading">
          <p>Verifying your payment...</p>
        </div>
      `;
    }

    try {
      const response = await fetch(`${CONFIG.backendUrl}/verify-payment`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          session_id: stripeSessionId
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();

      if (result.success && result.access_token) {
        console.log('[Paywall] Payment verified, unlocking content');

        // Store access token in cookie
        setCookie(CONFIG.accessCookiePrefix + articleSlug, result.access_token, CONFIG.cookieExpiryDays);

        // Clean URL (remove session_id)
        cleanUrl();

        // Unlock content
        unlockContent();

        // Show success message briefly
        showSuccessMessage();
      } else {
        throw new Error(result.error || 'Payment verification failed');
      }
    } catch (error) {
      console.error('[Paywall] Payment verification error:', error);

      // Show error in paywall prompt
      if (paywallPrompt) {
        paywallPrompt.innerHTML = `
          <div class="paywall-error">
            <h3>Payment Verification Failed</h3>
            <p>${error.message}</p>
            <p>If you were charged, please contact support with your receipt.</p>
            <button class="paywall-unlock-btn" onclick="location.reload()">Try Again</button>
          </div>
        `;
      }
    }
  }

  /**
   * Show a brief success message after unlocking
   */
  function showSuccessMessage() {
    const successDiv = document.createElement('div');
    successDiv.className = 'paywall-success';
    successDiv.style.cssText = `
      position: fixed;
      top: 20px;
      right: 20px;
      background: #16a34a;
      color: white;
      padding: 1rem 1.5rem;
      border-radius: 8px;
      box-shadow: 0 4px 12px rgba(0,0,0,0.15);
      z-index: 9999;
      animation: fadeIn 0.3s ease-out;
    `;
    successDiv.textContent = 'Payment successful! Article unlocked.';
    document.body.appendChild(successDiv);

    // Remove after 3 seconds
    setTimeout(() => {
      successDiv.style.opacity = '0';
      successDiv.style.transition = 'opacity 0.3s';
      setTimeout(() => successDiv.remove(), 300);
    }, 3000);
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
   * Remove token/session params from URL without reload
   */
  function cleanUrl() {
    const url = new URL(window.location);
    url.searchParams.delete('token');
    url.searchParams.delete('gift');
    url.searchParams.delete('session_id');
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
