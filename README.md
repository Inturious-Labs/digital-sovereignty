# Digital Sovereignty Chronicle

[![Deploy to Vercel](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml/badge.svg)](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml)

## Current Status (February 2026)

- ✅ **Live Site**: [https://digitalsovereignty.herbertyang.xyz](https://digitalsovereignty.herbertyang.xyz)
- ✅ **Platform**: Vercel (Hugo static site)
- ✅ **Latest Post**: "Enslaved by Data: There Is Something About Emma Stone" (Dec 30, 2025)
- ✅ **Auto-Deployment**: GitHub Actions → Vercel
- 📝 **Draft Ready**: "How To Top Up Proton Mail Balance with BTC" (Jan 31, 2026)
- 📊 **Archive**: 38 published articles (2025), 1 draft (2026)
- 🎯 **Focus**: Crypto, AI, Web3, decentralization, and data sovereignty

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

### 4. Publish

Run the publish script to validate and prepare your article:

```bash
dsc-publish
```

This script:
- Validates frontmatter, content, and images
- Prompts for publication date (defaults to today)
- Sets `draft: false`
- Shows git commands for committing

### 5. Commit and Create PR

```bash
git add .
git commit -m "Publish: My Article Title"
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
| `dsc-publish` | Validate, set date, set draft:false | Run from article folder |

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

```

## Deployment

Deployment is automated via GitHub Actions:

- **Trigger**: Push to `main` branch or PR merge
- **Schedule**: Daily at 12:00 UTC (checks for posts ready to publish)
- **Process**: Hugo build → Deploy to Vercel

The workflow automatically skips deployment if no posts are ready (date ≤ today AND draft = false).
