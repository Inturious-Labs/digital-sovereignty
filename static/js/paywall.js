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
    // Backend canister URL (IC mainnet - using raw domain to bypass certification)
    backendUrl: 'https://fhvra-iiaaa-aaaae-acznq-cai.raw.icp0.io',
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
    } else if (accessToken) {
      // Has stored access token - unlock locally (skip backend validation for offline/dev)
      console.log('[Paywall] Found stored access token, unlocking');
      unlockContent();
    } else if (sessionId) {
      // Has session - validate with backend
      validateAccess(articleSlug, null, sessionId);
    } else {
      console.log('[Paywall] No access token found, showing paywall');
    }

    // Set up unlock button click handler
    if (unlockButton) {
      unlockButton.addEventListener('click', function() {
        handleUnlockClick(articleSlug, price);
      });
    }

    // Set up gift button click handler
    const giftButton = document.getElementById('paywall-gift-btn');
    if (giftButton) {
      giftButton.addEventListener('click', function() {
        handleGiftClick(articleSlug);
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
   * Handle gift button click - show gift options modal
   */
  function handleGiftClick(articleSlug) {
    console.log('[Paywall] Gift clicked for:', articleSlug);

    // Get user's access token
    const accessToken = getCookie(CONFIG.accessCookiePrefix + articleSlug);
    if (!accessToken) {
      alert('You need to have purchased this article to gift it.');
      return;
    }

    // Create modal
    const modal = document.createElement('div');
    modal.id = 'gift-modal';
    modal.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0,0,0,0.5);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 10000;
    `;

    const articleTitle = document.querySelector('h1')?.textContent || articleSlug;

    modal.innerHTML = `
      <div style="
        background: white;
        padding: 2rem;
        border-radius: 12px;
        max-width: 400px;
        width: 90%;
        box-shadow: 0 4px 20px rgba(0,0,0,0.2);
      ">
        <h3 style="margin: 0 0 1rem 0; font-size: 1.25rem;">Gift this Article</h3>
        <p style="margin: 0 0 1.5rem 0; color: #666; font-size: 0.95rem;">
          Share "${articleTitle}" with a friend.
        </p>

        <div style="margin-bottom: 1.5rem;">
          <button id="gift-get-link" style="
            width: 100%;
            padding: 12px;
            background: #16a34a;
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 1rem;
            cursor: pointer;
            margin-bottom: 0.75rem;
          ">Get Shareable Link</button>

          <div id="gift-link-container" style="display: none; margin-top: 1rem;">
            <input id="gift-link-input" type="text" readonly style="
              width: 100%;
              padding: 10px;
              border: 1px solid #ddd;
              border-radius: 6px;
              font-size: 0.9rem;
              box-sizing: border-box;
            " />
            <button id="gift-copy-link" style="
              width: 100%;
              padding: 10px;
              background: #2563eb;
              color: white;
              border: none;
              border-radius: 6px;
              font-size: 0.9rem;
              cursor: pointer;
              margin-top: 0.5rem;
            ">Copy Link</button>
          </div>
        </div>

        <button id="gift-close" style="
          width: 100%;
          padding: 10px;
          background: #f3f4f6;
          color: #374151;
          border: none;
          border-radius: 6px;
          font-size: 0.9rem;
          cursor: pointer;
        ">Close</button>
      </div>
    `;

    document.body.appendChild(modal);

    // Event handlers
    document.getElementById('gift-close').onclick = () => modal.remove();
    modal.onclick = (e) => { if (e.target === modal) modal.remove(); };

    document.getElementById('gift-get-link').onclick = async () => {
      await generateGiftLink(articleSlug, accessToken, articleTitle);
    };
  }

  /**
   * Generate a gift link via backend
   */
  async function generateGiftLink(articleSlug, gifterToken, articleTitle) {
    const btn = document.getElementById('gift-get-link');
    btn.disabled = true;
    btn.textContent = 'Generating...';

    try {
      const response = await fetch(`${CONFIG.backendUrl}/create-gift`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          article_slug: articleSlug,
          gifter_token: gifterToken,
          article_title: articleTitle,
          recipient_email: null // Link option - no pre-set email
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();

      if (result.success && result.gift_url) {
        // Show the link
        const container = document.getElementById('gift-link-container');
        const input = document.getElementById('gift-link-input');
        input.value = result.gift_url;
        container.style.display = 'block';
        btn.style.display = 'none';

        // Set up copy button
        document.getElementById('gift-copy-link').onclick = () => {
          input.select();
          document.execCommand('copy');
          showSuccessMessage('Link copied to clipboard!');
        };
      } else {
        throw new Error(result.error || 'Failed to create gift link');
      }
    } catch (error) {
      console.error('[Paywall] Gift creation error:', error);
      alert('Unable to create gift link: ' + error.message);
      btn.disabled = false;
      btn.textContent = 'Get Shareable Link';
    }
  }

  /**
   * Handle gift token redemption
   */
  async function handleGiftRedemption(giftToken, articleSlug) {
    console.log('[Paywall] Redeeming gift token...');

    try {
      const response = await fetch(`${CONFIG.backendUrl}/redeem-gift`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          gift_token: giftToken,
          redeemer_email: '' // Optional - not required for redemption
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

        showSuccessMessage('Gift redeemed! Article unlocked.');
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

      // Convert price to cents (price is in dollars from data attribute)
      const priceCents = Math.round(parseFloat(price) * 100);

      // Call backend to create Stripe Checkout Session
      const response = await fetch(`${CONFIG.backendUrl}/create-checkout-session`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          article_slug: articleSlug,
          article_title: articleTitle,
          price_cents: priceCents,
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
  function showSuccessMessage(message = 'Payment successful! Article unlocked.') {
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
    successDiv.textContent = message;
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
