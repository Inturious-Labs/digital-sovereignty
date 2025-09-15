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

## Hugo Development Workflow

### Local Development Preview

To preview your site locally with correct localhost URLs, use the master writing workflow:

```bash
# Start local development server
./scripts/write preview
```

**Important**: Always use `./scripts/write preview` instead of `hugo serve` directly. The preview command automatically:
- Sets the correct localhost baseURL (`http://localhost:1313`)
- Includes draft posts (`-D` flag)
- Binds to all interfaces for network access
- Preserves production URLs in sitemap for SEO

### Other Writing Commands

```bash
# Create new post
./scripts/write new "Post Title" [category]

# Edit existing post
./scripts/write edit "Post Title"

# Publish post
./scripts/write publish "Post Title"

# Check session status
./scripts/write status
```

## Configure Custom Domains

1. Follow this guide to create `ALIAS`, `CNAME`, and `TXT` records on the domain registrar for the domain or subdomain:

https://internetcomputer.org/docs/building-apps/frontends/custom-domains/dns-setup

Note: for the subdomain `example`, this record `example.ic-domain.live.icp1.io` can only be created with a `CNAME` record, NOT the `ALIAS` record that the guide suggests.

2. Follow this guide to create `.well-known` folder,  `id-domains` file, and `.ic-assets.json5` file on the dfx folder. For a hugo site, the output folder is `public/`, which is regenerated every time hugo deploys. So these dfx-specific files should be placed in `static/` folder. All contents in `static/` are copied directly into `public/` when hugo re-deploys the site. 

https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains

3. Deploy the canister

4. Register the domain with the HTTP gateways by issuing the following command and replacing CUSTOM_DOMAIN with your custom domain:

```
curl -sL -X POST \
    -H 'Content-Type: application/json' \
    https://icp0.io/registrations \
    --data @- <<EOF
    {
      "name": "CUSTOM_DOMAIN"
    }
EOF
```

5. If the call is successful, you'll get a JSON response:

```
{"id":"REQUEST_ID"}
```

6. Track the progress of your registration with this command:

```
curl -sL -X GET \
    https://icp0.io/registrations/REQUEST_ID
```

7. Check the DNS record for the domain:

```
dig yourdomain.xyz CNAME
dig yourdomain.xyz TXT
dig yourdomain.xyz ALIAS
```