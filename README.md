# Digital Sovereignty Chronicle

[![Deploy to IC Mainnet](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml/badge.svg)](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml)

## Live Site

- **Production URL**: [https://digitalsovereignty.herbertyang.xyz](https://digitalsovereignty.herbertyang.xyz)
- **Internet Computer Canister**: [https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io](https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io)
- **Canister ID**: `wupbw-2aaaa-aaaae-abn7a-cai`

## Publishing Workflow

Complete workflow for creating and publishing a new article.

### 1. Create Draft Branch and Article Folder

```bash
# Create a new branch for your article
git checkout main
git pull
git checkout -b draft/my-article-slug

# Create the article folder (YYYY/MM/DD-slug format)
mkdir -p content/posts/2025/12/09-my-article-slug
cd content/posts/2025/12/09-my-article-slug
```

### 2. Initialize Article with Frontmatter

Run the interactive wizard to create `index.md`:

```bash
dsc-init-article
```

The wizard will prompt for:
- **Title**: Article title
- **Slug**: Auto-generated from folder name (can override)
- **Description**: SEO description
- **Category**: Choose from existing or create new
- **Series**: Optional, choose from existing or create new
- **Keywords**: Optional, comma-separated for SEO

This creates `index.md` with proper frontmatter and template structure.

### 3. Write Content

- Add your content to `index.md`
- Add images to the article folder (WebP format preferred)
- Add `featured-image.webp` for social media preview
- Preview locally: `hugo server -D` (from repo root)

### 4. Audit Before Publishing

Run the audit script to validate your article:

```bash
dsc-audit
```

The audit checks:
- Required frontmatter fields (title, date, slug, categories)
- Description length (SEO optimization)
- Word count
- Image references and files
- Placeholder content warnings

### 5. Finalize and Publish

When ready to publish:

```bash
# Update frontmatter
# - Change date from 2099-12-31 to actual publication date
# - Set draft: false

# Commit your changes
git add .
git commit -m "Publish: My Article Title"

# Push and create PR
git push -u origin draft/my-article-slug
gh pr create --base main --title "Publish: My Article Title"

# After PR is merged, clean up
git checkout main
git pull
git branch -d draft/my-article-slug
```

### Workflow Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `dsc-init-article` | Create index.md with frontmatter | Run from article folder |
| `dsc-audit` | Validate article before publish | Run from article folder |

## Git Branching Strategy

Simple branch-per-article workflow:

```
main (production)
 │
 ├── draft/article-a  ──> PR ──> merge ──> delete
 │
 ├── draft/article-b  ──> PR ──> merge ──> delete
 │
 └── draft/article-c  (work in progress)
```

**Key Principles:**
- `main` branch is always production-ready
- Each article gets its own `draft/slug` branch
- Create PR to merge into main when ready
- Delete branch after merge
- Multiple articles can be in progress simultaneously

## Content Structure

Articles use Hugo page bundles:

```
content/posts/
└── 2025/
    └── 12/
        └── 09-my-article-slug/
            ├── index.md           # Article content
            ├── featured-image.webp  # Social media preview
            └── other-images.webp  # Additional images
```

**Frontmatter Example:**
```yaml
---
title: "My Article Title"
date: 2025-12-09T12:00:00+08:00
slug: my-article-slug
draft: false
description: "A compelling description for SEO (50-160 chars)"
categories:
  - "crypto"
series:
  - "Deep Dive Series"
images: ["featured-image.webp"]
keywords: ["keyword1", "keyword2"]
enable_rapport: true
---
```

## Image Processing Scripts

Utility scripts for processing images (useful for Substack migration).

### HEIC to WebP Converter

```bash
./scripts/convert_heic_to_webp.sh content/posts/2025/12
```

Converts HEIC images to WebP and updates markdown references.

**Requirements**: `brew install imagemagick`

### Substack URL Updater

```bash
./scripts/update_substack_urls.sh content/posts/2025/12
```

Converts Substack CDN URLs to local file references.

### HEIC Image Remover

```bash
./scripts/remove_heic_images.sh
```

Safely removes HEIC files after conversion (requires confirmation).

## DFX Commands Reference

### Identity Management

```bash
dfx identity list
dfx identity use <identity-name>
dfx identity whoami
dfx identity get-principal
```

### Balance Checks

```bash
# ICP balance
dfx ledger --network ic balance

# Account ID (for receiving ICP)
dfx ledger --network ic account-id

# Cycles balance
dfx cycles --network ic balance
```

### Convert ICP to Cycles

```bash
dfx cycles convert --network ic --amount 1
```

## Deployment

Deployment is automated via GitHub Actions:

- **Trigger**: Push to `main` branch or PR merge
- **Schedule**: Daily at 12:00 UTC (checks for posts ready to publish)
- **Process**: Hugo build → Deploy to IC mainnet

The workflow automatically skips deployment if no posts are ready (date ≤ today AND draft = false).
