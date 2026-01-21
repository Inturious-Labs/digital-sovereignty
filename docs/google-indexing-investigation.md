# Google Search Console Indexing Investigation

**Date**: January 2026
**Issue**: Digital Sovereignty Chronicle has not been indexed by Google Search Console despite being live for over a year.

## Background

Three websites are hosted on Internet Computer (ICP) canisters:

| Site | Domain Type | Canister ID | GSC Indexed |
|------|-------------|-------------|-------------|
| herbertyang.xyz | Apex domain | `hbc6w-gqaaa-aaaag-aagdq-cai` | Yes |
| ic123.xyz | Apex domain | `bp6lc-ziaaa-aaaag-abqyq-cai` | Yes |
| digitalsovereignty.herbertyang.xyz | Subdomain | `wupbw-2aaaa-aaaae-abn7a-cai` | **No** |

All three sites have proper SEO configuration:
- robots.txt allowing all crawlers
- sitemap.xml generated
- Google site verification in place
- Meta tags (OpenGraph, Twitter Cards) configured

## Root Cause Analysis

### DNS Configuration Differences

Investigation revealed a critical difference in DNS setup:

**herbertyang.xyz (indexed)**
```
A Record:     23.236.116.77 (Zenlayer CDN)
TXT Record:   _canister-id.herbertyang.xyz → "hbc6w-gqaaa-aaaag-aagdq-cai"
```

**ic123.xyz (indexed)**
```
A Record:     23.236.116.77 (Zenlayer CDN)
TXT Record:   _canister-id.ic123.xyz → "bp6lc-ziaaa-aaaag-abqyq-cai"
```

**digitalsovereignty.herbertyang.xyz (NOT indexed)**
```
CNAME:        digitalsovereignty.herbertyang.xyz.icp1.io
              → boundary.dfinity.network
              → 23.142.184.129 (DFINITY boundary node)
TXT Record:   _canister-id.digitalsovereignty.herbertyang.xyz → "wupbw-2aaaa-aaaae-abn7a-cai"
CNAME:        _acme-challenge.digitalsovereignty.herbertyang.xyz.icp2.io (for SSL)
```

### Key Finding

The older sites (herbertyang.xyz, ic123.xyz) use:
- **Direct A record** pointing to Zenlayer CDN IP (`23.236.116.77`)
- Zenlayer is a traditional CDN that behaves like a standard web server

The newer subdomain setup (digitalsovereignty) uses:
- **CNAME chain** through DFINITY's boundary nodes (`*.icp1.io`)
- This is the current recommended ICP custom domain setup
- DFINITY boundary nodes have non-standard HTTP behavior that may confuse Googlebot

### Why Googlebot Struggles with DFINITY Boundary Nodes

1. **CNAME resolution chain** adds latency and complexity
2. **Non-standard HTTP responses** from boundary nodes
3. **Variable response times** that can timeout crawlers
4. **SSL certificate handling** through ACME challenge CNAME adds another layer

### ICP Infrastructure Evolution

DFINITY has made significant changes to ICP infrastructure over the years:
- The older `icp0.io` domain
- Introduction of `icp1.io` and `icp2.io` for custom domains
- Changes to boundary node architecture
- The Zenlayer CDN setup appears to be from an earlier era and may no longer be the recommended approach

## Decision

**Migrate from ICP canister hosting to Vercel.**

### Reasons

1. **Vercel uses standard HTTP infrastructure** that Google crawlers handle reliably
2. **Proven track record** - other projects (Readly, Prismatic, Subspend) work fine on Vercel
3. **Native Hugo support** - Vercel has built-in Hugo build integration
4. **Global CDN** with consistent response times
5. **Free tier** is sufficient for a blog
6. **Simpler deployment** - GitHub integration works out of the box

### What Won't Work

- **Cloudflare DNS alone** won't fix the issue - the problem is the origin server (ICP boundary nodes), not DNS resolution
- **Replicating the old Zenlayer setup** is undocumented and may no longer be supported by DFINITY

## Migration Plan

1. Create `vercel.json` configuration for Hugo
2. Connect GitHub repository to Vercel
3. Update DNS: point `digitalsovereignty.herbertyang.xyz` to Vercel
4. Remove or archive ICP-specific files (`.well-known/ic-domains`, `.ic-assets.json5`)
5. Update/remove GitHub Actions workflow (Vercel handles deploys)
6. Verify build and deployment
7. Submit sitemap to Google Search Console
8. Monitor indexing progress

## Files to Modify/Remove

### Remove (ICP-specific)
- `static/.well-known/ic-domains`
- `static/.ic-assets.json5`
- `dfx.json` (keep for reference or archive)
- `canister_ids.json` (keep for reference or archive)

### Modify
- `.github/workflows/deploy.yml` → Remove or repurpose for non-deployment tasks

### Add
- `vercel.json` - Vercel build configuration

## DNS Changes Required

Update DNS for `digitalsovereignty.herbertyang.xyz`:

| Current | New |
|---------|-----|
| CNAME → `*.icp1.io` | CNAME → `cname.vercel-dns.com` |
| Remove `_acme-challenge` CNAME | (Vercel handles SSL automatically) |
| Remove `_canister-id` TXT | (No longer needed) |

## Preserving ICP Option

The ICP canister (`wupbw-2aaaa-aaaae-abn7a-cai`) will continue to exist and can be accessed via:
- `https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io`

This serves as a backup and demonstrates the "digital sovereignty" aspect of the blog's theme - content exists on both centralized (Vercel) and decentralized (ICP) infrastructure.
