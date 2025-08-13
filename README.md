# README

## dfx commands

Switch to the correct identity:

```
dfx identity list
dfx identity use guizishanren
dfx identity whoami
```

Get the principal `lxmxz-3fjoo-5fcay-obm3a-3jk4a-r4ztf-sgvy5-w2pkk-ovwor-hf42s-7ae` for identity `guizishanren`:

```
dfx identity get-principal
```

Check the ICP balance and the account ID for the identity canister `guizishanren`:

```
dfx ledger --network ic balance
```

Check the account ID `0e5dc971adc229513ae59e5a8c83864dbf4d296d32306360ffd6bde154dab793` (for ICP transfer) for the identity canister `guizishanren`:

```
dfx ledger --network ic account-id
```

Check the cycles balance for the identity canister `guizishanren`:

```
dfx cycles --network ic balance
```

Transfer some $ICP into account `0e5dc...b793` and verify the balance has been topped up with `dfx ledger`. 

Then, convert $ICP into cycles from the ledger account to cycles account for this identity `guizishanren`:

```
dfx cycles convert --network ic --amount 1
```

Verify that the ICP balance has been deducted with `dfx ledger` and that cycles balance has been topped up with `dfx cycles`. 

## Content Management Scripts

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

