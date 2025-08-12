#!/bin/bash

# Script to update all Substack image URLs to local file references
# Usage: ./update_substack_urls.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to update markdown references from Substack URLs to local files
update_markdown_references() {
    local index_file="$1"
    local dir=$(dirname "$index_file")
    
    echo -e "${BLUE}Processing: ${index_file}${NC}"
    
    # Create backup
    cp "$index_file" "${index_file}.backup"
    
    # Find all image references in the markdown file
    # Pattern: [![](./filename.ext)](https://substackcdn.com/image/fetch/.../filename.ext)
    # We want to extract the filename and convert to: [![](./filename.ext)](./filename.ext)
    
    # Use a simpler, more reliable approach to replace Substack URLs
    # Pattern: [![](./filename)](https://substackcdn.com/image/fetch/...)
    # Convert to: ![](./filename)
    
    # Create a temporary file for processing
    cp "$index_file" "${index_file}.tmp"
    
    # Use awk for more reliable text processing
    awk '
    {
        # Find lines that match the pattern: [![](./filename)](https://substackcdn.com/image/fetch/...)
        if (match($0, /\[!\[\]\(\.\/[^)]*\)\]\(https:\/\/substackcdn\.com\/image\/fetch\/[^)]*\)/)) {
            # Extract the filename from the first part
            if (match($0, /\[!\[\]\(\.\/[^)]*\)\]/)) {
                filename = substr($0, RSTART + 5, RLENGTH - 6)
                # Replace the entire line with just the local image reference
                gsub(/\[!\[\]\(\.\/[^)]*\)\]\(https:\/\/substackcdn\.com\/image\/fetch\/[^)]*\)/, "![](./" filename ")")
            }
        }
        print
    }' "${index_file}.tmp" > "$index_file"
    
    # Clean up
    rm -f "${index_file}.tmp"
    
    echo -e "${GREEN}✓ Updated references in: ${index_file}${NC}"
}

# Function to process a single directory
process_directory() {
    local dir="$1"
    local updated_count=0
    
    echo -e "${BLUE}Processing directory: ${dir}${NC}"
    
    # Find index.md file in this directory
    local index_file="$dir/index.md"
    
    if [ -f "$index_file" ]; then
        if update_markdown_references "$index_file"; then
            ((updated_count++))
        fi
    else
        echo -e "${YELLOW}⚠ No index.md found in: ${dir}${NC}"
    fi
    
    echo -e "${GREEN}Directory ${dir}: ${updated_count} markdown files updated${NC}"
}

# Main execution
main() {
    echo -e "${BLUE}=== Substack URL to Local File Converter ===${NC}"
    echo "This script will:"
    echo "1. Find all index.md files in content/posts/2025/01/"
    echo "2. Update Substack image URLs to local file references"
    echo "3. Create backup files for all modified markdown files"
    echo ""
    
    # Check if we're in the right directory
    if [ ! -d "content/posts" ]; then
        echo -e "${RED}Error: content/posts directory not found.${NC}"
        echo "Please run this script from the project root directory."
        exit 1
    fi
    
    # Check if January folder exists
    if [ ! -d "content/posts/2025/01" ]; then
        echo -e "${RED}Error: content/posts/2025/01 directory not found.${NC}"
        echo "Please check if the January folder exists."
        exit 1
    fi
    
    # Find all directories containing index.md files in January
    markdown_dirs=$(find content/posts/2025/01 -name "index.md" -exec dirname {} \; | sort -u)
    
    if [ -z "$markdown_dirs" ]; then
        echo -e "${YELLOW}No index.md files found in content/posts/2025/01/${NC}"
        exit 0
    fi
    
    markdown_count=$(echo "$markdown_dirs" | wc -l)
    echo -e "${GREEN}Found index.md files in ${markdown_count} directories in January${NC}"
    echo ""
    
    # Process each directory
    total_updated=0
    
    while IFS= read -r dir; do
        if [ -n "$dir" ]; then
            process_directory "$dir"
            echo ""
        fi
    done <<< "$markdown_dirs"
    
    echo -e "${GREEN}=== URL Update Complete ===${NC}"
    echo -e "${GREEN}Total markdown files updated: ${total_updated}${NC}"
    echo ""
    echo -e "${YELLOW}Note: Backup files (.backup) were created for all modified markdown files.${NC}"
    echo -e "${BLUE}This was a test run on January folder only.${NC}"
}

# Run main function
main "$@"
