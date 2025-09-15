#!/bin/bash

# Digital Sovereignty Chronicle - Auto Publisher
# Usage: ./scripts/publish-post.sh "Post Title" [commit-message]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
POSTS_DIR="content/posts"
MAIN_BRANCH="main"

# Check if title is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}Usage: $0 \"Post Title\" [commit-message]${NC}"
    echo "Example: $0 \"My New Post\" \"Add new crypto analysis post\""
    exit 1
fi

# Get parameters
TITLE="$1"
COMMIT_MSG="${2:-\"Publish: $TITLE\"}"

# Generate slug from title
SLUG=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | sed 's/^-\|-$//g')

# Find the post file
POST_FILE=$(find "$POSTS_DIR" -name "index.md" -exec grep -l "slug: $SLUG" {} \; | head -1)

if [ -z "$POST_FILE" ]; then
    echo -e "${RED}❌ Post with slug '$SLUG' not found${NC}"
    echo "Available posts:"
    find "$POSTS_DIR" -name "index.md" -exec grep -H "title:" {} \; | sed 's/.*title: "\(.*\)".*/  - \1/'
    exit 1
fi

POST_DIR=$(dirname "$POST_FILE")

echo -e "${BLUE}📝 Publishing: $TITLE${NC}"
echo -e "${BLUE}📁 Location: $POST_DIR${NC}"

# Check if post is still in draft
if grep -q "draft: true" "$POST_FILE"; then
    echo -e "${YELLOW}⚠️  Post is marked as draft. Setting draft: false${NC}"
    sed -i '' 's/draft: true/draft: false/' "$POST_FILE"
fi

# Verify required fields
echo -e "${BLUE}🔍 Verifying post metadata...${NC}"
if ! grep -q "title:" "$POST_FILE"; then
    echo -e "${RED}❌ Missing title in frontmatter${NC}"
    exit 1
fi

if ! grep -q "date:" "$POST_FILE"; then
    echo -e "${RED}❌ Missing date in frontmatter${NC}"
    exit 1
fi

if ! grep -q "categories:" "$POST_FILE"; then
    echo -e "${YELLOW}⚠️  No categories found${NC}"
fi

# Check for images
IMAGE_COUNT=$(find "$POST_DIR" -name "*.webp" -o -name "*.jpg" -o -name "*.png" | wc -l)
echo -e "${BLUE}🖼️  Found $IMAGE_COUNT image(s) in post directory${NC}"

# Git operations
echo -e "${BLUE}🔄 Starting git operations...${NC}"

# Make sure we're on main branch
git checkout "$MAIN_BRANCH" 2>/dev/null || {
    echo -e "${YELLOW}⚠️  Creating main branch${NC}"
    git checkout -b "$MAIN_BRANCH"
}

# Add all files in the post directory
git add "$POST_DIR"

# Check if there are changes to commit
if git diff --staged --quiet; then
    echo -e "${YELLOW}⚠️  No changes to commit for this post${NC}"
else
    # Commit the changes
    echo -e "${BLUE}💾 Committing changes...${NC}"
    git commit -m "$COMMIT_MSG"
    
    echo -e "${BLUE}📤 Pushing to remote...${NC}"
    git push origin "$MAIN_BRANCH"
    
    echo -e "${GREEN}✅ Post published successfully!${NC}"
    echo -e "${GREEN}🚀 GitHub Actions will deploy automatically${NC}"
    echo -e "${GREEN}📧 RSS feed will update and trigger Buttondown${NC}"
fi

# Show post URL
POST_SLUG=$(grep "slug:" "$POST_FILE" | sed 's/slug: //' | tr -d ' ')
echo -e "${GREEN}🌐 Post URL: https://digitalsovereignty.herbertyang.xyz/p/$POST_SLUG${NC}"

echo -e "${BLUE}📊 Post summary:${NC}"
echo -e "   Title: $(grep 'title:' "$POST_FILE" | sed 's/title: "//' | sed 's/"//')"
echo -e "   Category: $(grep -A1 'categories:' "$POST_FILE" | tail -1 | sed 's/.*"\(.*\)".*/\1/')"
echo -e "   Word count: $(wc -w < "$POST_FILE" | tr -d ' ') words"
echo -e "   Images: $IMAGE_COUNT"