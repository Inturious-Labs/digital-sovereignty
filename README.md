# Digital Sovereignty Chronicle

A Hugo static site published at
[digitalsovereignty.herbertyang.xyz](https://digitalsovereignty.herbertyang.xyz),
deployed on Vercel, with articles delivered to subscribers by Buttondown.

## How Publishing Works

Articles publish themselves on a date you choose. You do not need to be at your
machine when an article goes live, and nothing has to be merged on the day.

Three pieces make that work:

1. **The date decides.** The production build runs `hugo --minify --gc` *without*
   the `-F` flag, so Hugo omits any article whose `date` is still in the future.
   A future-dated article is absent from the site *and* the RSS feed — not hidden,
   genuinely not built.
2. **A daily rebuild checks.** A Vercel cron job runs every day at 14:00 UTC
   (22:00 Shanghai) and triggers a rebuild. Whatever has crossed its date gets
   published; everything else stays invisible.
3. **Buttondown notices once.** It polls the RSS feed every 30 minutes and dedupes
   on `<guid>` (the article permalink), so a rebuild that changes nothing sends
   nothing, and each article can only ever produce one email.

**Daily rebuild, any cadence you like.** The cron is a heartbeat, not the schedule.
You control cadence entirely through each article's `date`. A missed cron run
delays an article by hours, not by a week — which is why the rebuild is daily even
if you publish weekly.

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

### 4. Preview Locally

Start the Hugo development server from the repo root:

```bash
hugo serve -D -F
```

Then open **<http://localhost:1313/>** in your browser. Your article appears at
`http://localhost:1313/p/<your-slug>/`.

Both flags matter for unpublished work:

| Flag | Meaning | Why you need it |
|------|---------|-----------------|
| `-D` | Include drafts | New articles have `draft: true` until `dsc-publish` runs |
| `-F` | Include future-dated posts | `dsc-init-article` sets a placeholder date far in the future |

Without them, Hugo builds the site but your new article is **silently missing** —
no error, it just never appears in the list.

The server watches for changes and live-reloads the browser as you edit
`index.md`. Press `Ctrl+C` to stop it.

**Troubleshooting:**

- **Article not showing** — you almost certainly dropped `-D` or `-F`.
- **Port 1313 already in use** — another server is still running. Stop it with
  `pkill -f "hugo serve"`, or pick another port: `hugo serve -D -F -p 1314`.
- **Edits not appearing** — Fast Render Mode can miss some changes. Restart with
  `hugo serve -D -F --disableFastRender`.
- **`hugo: command not found`** — install it: `brew install hugo`.

### 5. Publish

Run the publish script to validate and prepare your article:

```bash
dsc-publish
```

This script:
- Validates frontmatter, content, and images
- **Asks when to publish** — `now`, or a future date to schedule it
- Sets `draft: false`
- Shows git commands for committing

```
Publish date [now | YYYY-MM-DD]: 2026-09-22
[OK] Date set to: 2026-09-22T22:00:00+08:00
[i]  Scheduled - stays hidden until 2026-09-22, then publishes on the daily rebuild.
```

The prompt is required — there is no silent default, so an article cannot be
published on the wrong day by accident. Past dates are rejected: Buttondown skips
items dated more than a day before it discovers them, so a backdated article would
never reach subscribers.

**`now` and today's date are not the same thing:**

| You type | Date written | Goes live |
|----------|--------------|-----------|
| `now` | current timestamp | immediately, on the next build |
| today's date | today at `22:00` | tonight, when the cron runs |
| a future date | that day at `22:00` | on that day |

Both are legitimate — typing today's date is how you publish "tonight at 22:00",
which lines up with the daily rebuild. To go live right away, answer `now`.
The script tells you which one you picked.

See [Scheduled Publishing](#scheduled-publishing) for the details.

### 6. Commit and Create PR

```bash
# Stage the article path explicitly — never `git add .`, which can sweep in
# unrelated drafts sitting untracked in content/
git add content/posts/YYYY/MM/DD-my-article-slug
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

**Installation (symlinks to /usr/local/bin):**

```bash
sudo ln -sf /Users/zire/matrix/zire/digital-sovereignty/scripts/dsc-init-article /usr/local/bin/dsc-init-article
sudo ln -sf /Users/zire/matrix/zire/digital-sovereignty/scripts/dsc-publish /usr/local/bin/dsc-publish
```

## Scheduled Publishing

Reference for the mechanism described in [How Publishing Works](#how-publishing-works).

### Queueing a backlog

Set each article a week apart, merge them all at once, and leave them alone:

```
article A -> 2026-09-01    article B -> 2026-09-08    article C -> 2026-09-15
```

All three can live on `main` from day one. Only the one whose date has arrived is
built, so subscribers receive them one at a time.

### The two states

`draft: true` and a future `date` both hide an article, but only one of them ever
un-hides itself:

| Frontmatter | Built today? | Publishes when |
|-------------|--------------|----------------|
| `draft: true` | no | never, until you flip the flag by hand |
| `draft: false` + future `date` | no | automatically, on that date |
| `draft: false` + past date | yes | already live |

Scheduling relies on the date, so a scheduled article is always `draft: false`.
Use `draft: true` for work that genuinely is not finished.

### Rules

- **Space articles at least a day apart** so only one enters the feed per rebuild.
- **Never change the slug of an article in the newest 10** (the RSS window). The
  slug becomes the `<guid>`, so a new slug reads as a brand-new post and Buttondown
  will send it again. Editing a title or body will not cause a second send — the
  guid does not move. But do not read that as "an edit gets picked up": Buttondown
  reads each guid exactly once, so an edit **cannot** change or trigger an email.
  See [Recovering an article Buttondown refused](#recovering-an-article-buttondown-refused).
- **Do not backdate.** Buttondown's "skip old items" setting ignores anything dated
  more than a day before discovery, so a backdated article silently never sends.
  `dsc-publish` rejects past dates for this reason.
- **Expect a fuzzy minute.** On the Hobby plan Vercel fires the cron at a random
  minute within the scheduled hour, so publication lands somewhere in
  22:00–22:59 Shanghai.

### Postponing an article

Change the date and merge again. As long as the original date has not yet passed,
nothing was ever built, so nothing was sent.

### Configuration

| Where | Setting |
|-------|---------|
| `vercel.json` | `crons` → `/api/rebuild` on `0 14 * * *` (UTC) |
| `api/rebuild.js` | Verifies `CRON_SECRET`, then POSTs to the deploy hook |
| Vercel env vars | `CRON_SECRET`, `DEPLOY_HOOK_URL` (Production) |
| Buttondown | "Skip old items" enabled; RSS-to-email on the site feed |

`CRON_SECRET` is the shared secret Vercel sends as `Authorization: Bearer …`;
`/api/rebuild` returns 401 to anything else. `DEPLOY_HOOK_URL` is the deploy hook
from **Project → Settings → Git → Deploy Hooks**. Both are secrets — anyone holding
the hook URL can trigger builds.

### If an article does not appear

1. **Check the cron ran** — Vercel → Settings → Cron Jobs → View Logs. A 401 means
   `CRON_SECRET` does not match; a 500 means `DEPLOY_HOOK_URL` is unset for Production.
2. **Check the date has actually passed** in Shanghai time, not UTC.
3. **Check `draft: false`** — a lingering `draft: true` blocks it regardless of date.
4. **Rebuild locally the way production does:** `hugo --gc` with no `-F`. If the
   article is missing there, it will be missing on the site.

To preview scheduled work locally, use `hugo serve -D -F` — `-F` includes
future-dated posts, `-D` includes drafts.

### Recovering an article Buttondown refused

Buttondown screens incoming items for prohibited keywords — the names of some
crypto exchanges are on that list. When an item trips the filter you get an email
saying the send failed. **Nothing is saved.** Buttondown discards the item rather
than keeping a draft you can fix, but it still records the guid as seen, so it
never reads that URL again.

The consequence is the part that costs a day: **editing the article does not fix
it.** The site updates, the offending word is gone, and no email will ever be sent
for that URL. Confirmed 2026-09-02 — the refused article had no email object in
the account in any status (draft, blocked, errored), while the guid stayed
suppressed.

Two ways out:

1. **Give the article a new address.** Change the `slug`, set `date` to earlier
   today, and add an `aliases` entry so the old URL redirects instead of 404ing:

   ```yaml
   slug: where-to-buy-btc-in-2026
   aliases:
     - "/p/where-to-buy-btc/"
   ```

   The new slug is a new guid, so Buttondown treats it as a fresh post. Set the
   date to a time that has **already passed** — a future timestamp makes Hugo omit
   the article from the build entirely, and it fails silently. Keep it same-day so
   "skip old items" does not reject it as stale. Edit the frontmatter by hand for
   this; `dsc-publish` always writes 22:00, which is still ahead of you for most
   of the working day.

   Confirm with a production-equivalent build before merging — `hugo --gc` with no
   `-F`. The page should exist at the new slug, the old path should contain a
   redirect, and the feed's top `<guid>` should be the new URL.

2. **Create and send the email directly through the API**, bypassing RSS. Leaves
   the URL untouched, but you compose the email yourself.

Check what Buttondown actually holds before assuming anything — note that
`GET /v1/emails` returns only **sent** emails unless you ask for another status:

```bash
set -a; . ~/.config/buttondown.env; set +a   # key lives here on Moonglade, mode 600
curl -s -H "Authorization: Token $BUTTONDOWN_API_KEY" \
  "https://api.buttondown.com/v1/emails?status=draft" | python3 -m json.tool
```

The auth scheme is `Token`, not `Bearer`, and the host is `api.buttondown.com`.

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

Deployment is handled entirely by Vercel — there are no GitHub Actions in this repo.

| Trigger | What happens |
|---------|--------------|
| Push or PR merge to `main` | Vercel builds and deploys automatically |
| Daily cron, 14:00 UTC | `/api/rebuild` fires a deploy hook; anything that has reached its date goes live |

The build command is `hugo --minify --gc` (see `vercel.json`). It deliberately
omits `-F`, which is what makes future-dated articles invisible until their day
arrives.
