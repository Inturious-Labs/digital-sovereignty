#!/bin/bash

# Digital Sovereignty Chronicle - New Post Creator
# Usage: ./scripts/new-post.sh "Post Title" [category] [series]

set -e

# Configuration
POSTS_DIR="content/posts"
TEMPLATE_FILE=".templates/post-template.md"
CURRENT_YEAR=$(date +%Y)
CURRENT_MONTH=$(date +%m)
CURRENT_DAY=$(date +%d)
CURRENT_DATE=$(date -u +"%Y-%m-%dT%H:%M:%S+00:00")

# Check if title is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 \"Post Title\" [category] [series]"
    echo "Example: $0 \"My New Post\" \"crypto\" \"weekly-updates\""
    exit 1
fi

# Get parameters
TITLE="$1"
CATEGORY="${2:-general}"
SERIES="${3:-}"

# Generate slug from title
SLUG=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | sed 's/^-\|-$//g')

# Create directory structure
POST_DIR="$POSTS_DIR/$CURRENT_YEAR/$CURRENT_MONTH/$CURRENT_DAY-$SLUG"
mkdir -p "$POST_DIR"

# Create post file from template
POST_FILE="$POST_DIR/index.md"

if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Template file not found: $TEMPLATE_FILE"
    exit 1
fi

# Replace template variables
sed "s|{{ TITLE }}|$TITLE|g; \
     s|{{ DATE }}|$CURRENT_DATE|g; \
     s|{{ SLUG }}|$SLUG|g; \
     s|{{ DESCRIPTION }}|Brief description of the post|g; \
     s|{{ CATEGORY }}|$CATEGORY|g; \
     s|{{ SERIES }}|$SERIES|g; \
     s|{{ CONTENT_START }}||g" "$TEMPLATE_FILE" > "$POST_FILE"

# If no series specified, remove the series line
if [ -z "$SERIES" ]; then
    sed -i '' '/series:/,/^$/d' "$POST_FILE"
fi

echo "✅ New post created:"
echo "   📁 Directory: $POST_DIR"
echo "   📝 File: $POST_FILE"
echo "   🏷️  Category: $CATEGORY"
if [ -n "$SERIES" ]; then
    echo "   📚 Series: $SERIES"
fi
echo ""
echo "📋 Next steps:"
echo "   1. Add images to: $POST_DIR/"
echo "   2. Edit the post: $POST_FILE"
echo "   3. Run 'npm run preview' to preview locally"
echo "   4. When ready: './scripts/publish-post.sh \"$TITLE\"'"

# Open the post file in the default editor if EDITOR is set
if [ -n "${EDITOR:-}" ]; then
    $EDITOR "$POST_FILE"
elif command -v cursor > /dev/null 2>&1; then
    cursor "$POST_FILE"
elif command -v code > /dev/null 2>&1; then
    code "$POST_FILE"
else
    echo "💡 Tip: Set EDITOR environment variable or install Cursor/VS Code for auto-opening"
fi