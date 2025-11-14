# README

[![Deploy to IC Mainnet](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml/badge.svg)](https://github.com/Inturious-Labs/digital-sovereignty/actions/workflows/deploy.yml)

## Live Site

- **Production URL**: [https://digitalsovereignty.herbertyang.xyz](https://digitalsovereignty.herbertyang.xyz)
- **Internet Computer Canister**: [https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io](https://wupbw-2aaaa-aaaae-abn7a-cai.icp0.io)
- **Canister ID**: `wupbw-2aaaa-aaaae-abn7a-cai`

## dfx commands

Switch to the correct identity:

```
dfx identity list
dfx identity use <identity-name>
dfx identity whoami
```

Get the principal for your identity:

```
dfx identity get-principal
```

Check the ICP balance and the account ID for your identity:

```
dfx ledger --network ic balance
```

Check the account ID (for ICP transfer) for your identity:

```
dfx ledger --network ic account-id
```

Check the cycles balance for your identity:

```
dfx cycles --network ic balance
```

Transfer some $ICP into your account and verify the balance has been topped up with `dfx ledger`.

Then, convert $ICP into cycles from the ledger account to cycles account for your identity:

```
dfx cycles convert --network ic --amount 1
```

Verify that the ICP balance has been deducted with `dfx ledger` and that cycles balance has been topped up with `dfx cycles`.

## Substack Migration Scripts

This project includes several utility scripts for managing content and converting image formats. All scripts are located in the `scripts/` directory.

### 1. HEIC to WebP Converter (`convert_heic_to_webp.sh`)

**Purpose**: Converts HEIC images to WebP format and updates markdown references.

**What it does**:
- Finds all HEIC images in a specified folder
- Converts them to WebP format using ImageMagick
- Updates markdown references from HEIC to WebP filenames
- Creates backup files before making changes
- Optionally removes original HEIC files

**Usage**:
```bash
# Convert HEIC files in January folder
./scripts/convert_heic_to_webp.sh content/posts/2025/01

# Convert HEIC files in February folder
./scripts/convert_heic_to_webp.sh content/posts/2025/02

# Convert HEIC files in any specific folder
./scripts/convert_heic_to_webp.sh content/posts/2025/03
```

**Requirements**: ImageMagick must be installed (`brew install imagemagick`)

### 2. Substack URL Updater (`update_substack_urls.sh`)

**Purpose**: Converts Substack CDN image URLs to local file references.

**What it does**:
- Finds all markdown files in a specified folder
- Converts Substack CDN URLs to local file references
- Creates backup files before making changes
- Handles complex URL patterns automatically

**Usage**:
```bash
# Update URLs in January folder
./scripts/update_substack_urls.sh content/posts/2025/01

# Update URLs in February folder
./scripts/update_substack_urls.sh content/posts/2025/02

# Update URLs in any specific folder
./scripts/update_substack_urls.sh content/posts/2025/03
```

### 3. HEIC Image Remover (`remove_heic_images.sh`)

**Purpose**: Safely removes all HEIC images from the content directory.

**What it does**:
- Recursively searches for all HEIC files in `content/posts/`
- Shows exactly what files will be deleted
- Requires explicit confirmation before deletion
- Provides detailed feedback on the deletion process

**Usage**:
```bash
./scripts/remove_heic_images.sh
```

**Safety**: Requires typing "yes" to confirm deletion

### 4. Typical Workflow

**For processing new content**:
```bash
# 1. Convert HEIC images to WebP
./scripts/convert_heic_to_webp.sh content/posts/2025/01

# 2. Update Substack URLs to local references
./scripts/update_substack_urls.sh content/posts/2025/01

# 3. (Optional) Remove original HEIC files after verification
./scripts/remove_heic_images.sh
```

**Benefits**:
- **Web-optimized images**: WebP format provides better compression
- **Local references**: No dependency on external CDN services
- **Batch processing**: Process entire folders at once
- **Safe operations**: Backup files and confirmation prompts
- **Flexible**: Specify any folder path as needed

**Note**: Always run these scripts from the project root directory (`digital-sovereignty/`).

## Git Branching Strategy

This project uses a specialized branching strategy designed for continuous content creation and publication:

### Branch Structure

```
main (production)
 │
 ├── drafts/writing-pad (long-lived drafts branch)
 │    │
 │    ├── publish/article-name-1 (ephemeral publish branch)
 │    │    └──> merges into main, then deleted
 │    │
 │    ├── publish/article-name-2 (ephemeral publish branch)
 │    │    └──> merges into main, then deleted
 │    │
 │    └── (continues as "moving train" of drafts)
```

### Visual Workflow

```
main:           o─────o──────o──────o──────o
                      ↑      ↑      ↑
                      │      │      │
                      │      │      └─ publish/article-3 (merge & delete)
                      │      └──────── publish/article-2 (merge & delete)
                      └─────────────── publish/article-1 (merge & delete)
                                       │
                                       │
drafts/writing-pad:  o───o───o───o───┤───o───o───o (continuous)
                     │       │       │       │
                     │       │       │       └─ new draft started
                     │       │       └───────── article-3 ready
                     │       └─────────────── article-2 ready
                     └───────────────────── article-1 ready
```

### The "Moving Train" Concept

The `drafts/writing-pad` branch acts as a **moving train** where:
- Multiple articles exist in various stages of completion
- Some drafts are just starting
- Some drafts are nearly ready for publication
- Published articles "graduate" and leave the train
- New articles join the train as new drafts
- The train (branch) itself **never gets merged or deleted**

### Publishing Workflow

When an article is ready to publish:

1. **Create a publish branch** from `drafts/writing-pad`:
   ```bash
   git checkout drafts/writing-pad
   git checkout -b publish/article-name
   ```

2. **Push and create PR**:
   ```bash
   git push -u origin publish/article-name
   gh pr create --title "Publish: Article Name" --base main
   ```

3. **Merge and delete** the publish branch:
   ```bash
   # Merge PR on GitHub (this deletes the publish branch automatically)
   ```

4. **Return to drafts branch**:
   ```bash
   git checkout drafts/writing-pad
   git branch -D publish/article-name  # Clean up local branch
   ```

5. **Continue working** on other drafts in `drafts/writing-pad`

### Key Principles

- ✅ `main` branch remains pristine and production-ready
- ✅ `drafts/writing-pad` is a **long-lived branch** that never gets merged
- ✅ Each publish branch is **ephemeral** and gets deleted after merging
- ✅ Multiple drafts can coexist at different stages of completion
- ✅ No risk of losing draft work when publishing individual articles
- ✅ Clean separation between draft work and production releases

### Why This Approach?

**Traditional branching**: Each feature/article would require its own branch from main, making it difficult to work on multiple drafts simultaneously.

**This approach**: The `drafts/writing-pad` branch serves as a persistent workspace where multiple articles can be developed in parallel, with individual articles "graduating" to production when ready.

This mirrors a physical writing pad where you might have several drafts at various stages, and you tear off pages to publish them when they're ready, while the pad itself remains intact for future work.