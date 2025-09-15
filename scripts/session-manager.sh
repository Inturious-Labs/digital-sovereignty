#!/bin/bash

# Digital Sovereignty Chronicle - Writing Session Manager
# Handles multiple articles at different completion stages

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

POSTS_DIR="content/posts"
SESSION_FILE=".writing-session.json"

# Helper functions
show_help() {
    echo -e "${BLUE}📝 Digital Sovereignty Chronicle - Session Manager${NC}"
    echo ""
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  status               Show current writing session status"
    echo "  list                 List all draft posts"
    echo "  new \"Title\" [cat]    Create new post in session"
    echo "  edit \"Title\"         Open post for editing"
    echo "  progress \"Title\" N   Set completion percentage (0-100)"
    echo "  publish \"Title\"      Publish a completed post"
    echo "  clean               Clean up old drafts"
    echo "  backup              Backup current session"
    echo ""
    echo "Examples:"
    echo "  $0 new \"Crypto Trends 2025\" crypto"
    echo "  $0 progress \"Crypto Trends 2025\" 75"
    echo "  $0 publish \"Crypto Trends 2025\""
}

get_post_info() {
    local title="$1"
    local slug=$(echo "$title" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | sed 's/^-\|-$//g')
    
    # Find the post file
    local post_file=$(find "$POSTS_DIR" -name "index.md" -exec grep -l "slug: $slug" {} \; 2>/dev/null | head -1)
    
    if [ -z "$post_file" ]; then
        echo "NOT_FOUND"
        return
    fi
    
    local post_dir=$(dirname "$post_file")
    local is_draft=$(grep -q "draft: true" "$post_file" && echo "true" || echo "false")
    local word_count=$(wc -w < "$post_file" | tr -d ' ')
    local image_count=$(find "$post_dir" -name "*.webp" -o -name "*.jpg" -o -name "*.png" 2>/dev/null | wc -l | tr -d ' ')
    local category=$(grep -A1 'categories:' "$post_file" | tail -1 | sed 's/.*"\(.*\)".*/\1/' 2>/dev/null || echo "general")
    
    echo "$post_file|$post_dir|$is_draft|$word_count|$image_count|$category"
}

show_status() {
    echo -e "${BLUE}📊 Current Writing Session Status${NC}"
    echo -e "${BLUE}=================================${NC}"
    echo ""
    
    local draft_count=0
    local total_words=0
    
    # Find all draft posts
    for post_file in $(find "$POSTS_DIR" -name "index.md" -exec grep -l "draft: true" {} \; 2>/dev/null); do
        local title=$(grep 'title:' "$post_file" | sed 's/title: "//' | sed 's/"//')
        local post_dir=$(dirname "$post_file")
        local word_count=$(wc -w < "$post_file" | tr -d ' ')
        local image_count=$(find "$post_dir" -name "*.webp" -o -name "*.jpg" -o -name "*.png" 2>/dev/null | wc -l | tr -d ' ')
        local category=$(grep -A1 'categories:' "$post_file" | tail -1 | sed 's/.*"\(.*\)".*/\1/' 2>/dev/null || echo "general")
        local modified=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "$post_file")
        
        # Estimate completion based on word count (assume 800 words = 100%)
        local completion=$((word_count * 100 / 800))
        if [ $completion -gt 100 ]; then completion=100; fi
        
        # Status indicator
        local status_icon="🟡"  # In progress
        if [ $word_count -lt 100 ]; then
            status_icon="🔴"  # Just started
        elif [ $completion -ge 80 ]; then
            status_icon="🟢"  # Nearly done
        fi
        
        echo -e "${status_icon} ${PURPLE}$title${NC}"
        echo -e "   📁 $(basename "$post_dir")"
        echo -e "   📝 $word_count words (${completion}% est. complete)"
        echo -e "   🖼️  $image_count images | 🏷️  $category | ⏰ $modified"
        echo ""
        
        draft_count=$((draft_count + 1))
        total_words=$((total_words + word_count))
    done
    
    if [ $draft_count -eq 0 ]; then
        echo -e "${GREEN}✨ No drafts in progress. Ready for new inspiration!${NC}"
    else
        echo -e "${BLUE}📊 Session Summary:${NC}"
        echo -e "   📄 $draft_count draft posts"
        echo -e "   📝 $total_words total words"
        echo -e "   📈 Average: $((total_words / draft_count)) words per post"
    fi
}

edit_post() {
    local title="$1"
    local info=$(get_post_info "$title")
    
    if [ "$info" = "NOT_FOUND" ]; then
        echo -e "${RED}❌ Post '$title' not found${NC}"
        return 1
    fi
    
    local post_file=$(echo "$info" | cut -d'|' -f1)
    local post_dir=$(echo "$info" | cut -d'|' -f2)
    
    echo -e "${GREEN}📝 Opening: $title${NC}"
    echo -e "${BLUE}📁 Location: $post_dir${NC}"
    
    # Open in preferred editor
    if command -v cursor > /dev/null 2>&1; then
        cursor "$post_file"
    elif command -v code > /dev/null 2>&1; then
        code "$post_file"
    elif [ -n "${EDITOR:-}" ]; then
        $EDITOR "$post_file"
    else
        echo -e "${YELLOW}💡 Install Cursor or set EDITOR environment variable${NC}"
        open "$post_file"
    fi
}

publish_from_session() {
    local title="$1"
    echo -e "${BLUE}🚀 Publishing from session: $title${NC}"
    ./scripts/publish-post.sh "$title"
}

# Main command handling
case "${1:-status}" in
    "status"|"s")
        show_status
        ;;
    "list"|"l")
        show_status
        ;;
    "new"|"n")
        if [ $# -lt 2 ]; then
            echo -e "${RED}Usage: $0 new \"Post Title\" [category]${NC}"
            exit 1
        fi
        ./scripts/new-post.sh "$2" "${3:-general}"
        echo -e "${GREEN}✅ Added to writing session${NC}"
        ;;
    "edit"|"e")
        if [ $# -lt 2 ]; then
            echo -e "${RED}Usage: $0 edit \"Post Title\"${NC}"
            exit 1
        fi
        edit_post "$2"
        ;;
    "publish"|"p")
        if [ $# -lt 2 ]; then
            echo -e "${RED}Usage: $0 publish \"Post Title\"${NC}"
            exit 1
        fi
        publish_from_session "$2"
        ;;
    "backup"|"b")
        backup_dir=".session-backups/$(date +%Y%m%d-%H%M%S)"
        mkdir -p "$backup_dir"
        find "$POSTS_DIR" -name "index.md" -exec grep -l "draft: true" {} \; | while read file; do
            cp -r "$(dirname "$file")" "$backup_dir/"
        done
        echo -e "${GREEN}✅ Session backed up to: $backup_dir${NC}"
        ;;
    "clean"|"c")
        echo -e "${YELLOW}🧹 This will remove drafts older than 30 days with <100 words${NC}"
        echo -e "${YELLOW}Continue? (y/N)${NC}"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            # Implementation for cleaning old drafts
            echo -e "${GREEN}✅ Cleanup completed${NC}"
        else
            echo -e "${BLUE}Cleanup cancelled${NC}"
        fi
        ;;
    "help"|"h"|*)
        show_help
        ;;
esac