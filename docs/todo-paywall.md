# Paywall TODO

1. [ ] Test end-to-end flow with Stripe test keys (IN PROGRESS)
2. [ ] Configure Resend email API key
3. [ ] Set up CI/CD for backend canister deployment
4. [ ] Switch to production Stripe keys when ready to go live

## Key Info
- Canister ID: `fhvra-iiaaa-aaaae-acznq-cai`
- Canister URL: `https://fhvra-iiaaa-aaaae-acznq-cai.raw.icp0.io`
- Test URL: https://digitalsovereignty.herbertyang.xyz/posts/paywall-test/
- Test card: `4242 4242 4242 4242` (any future expiry, any CVC)

## Configuration Endpoints
```bash
# Stripe (already configured with test keys)
dfx canister call dsc-backend stripe_configure '("sk_test_...", "whsec_...")' --network ic

# Resend email
dfx canister call dsc-backend email_configure '("re_...")' --network ic

# HMAC secret (for session signing)
dfx canister call dsc-backend auth_configure '("your-secure-secret")' --network ic

# Site URL (already defaults to digitalsovereignty.herbertyang.xyz)
dfx canister call dsc-backend site_configure '("https://digitalsovereignty.herbertyang.xyz")' --network ic
```

## Recent Commits
- `3d7d784` - Add dynamic pricing to gift emails and legacy payment flow
- `cfb9344` - Remove hardcoded values and add configuration endpoints
- `a0ecdf9` - Add per-article pricing support
- `e0d8270` - Simplify paywall preview using `<!--more-->` separator
