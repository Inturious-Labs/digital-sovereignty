#!/bin/bash

# Script to remove all .md.backup files recursively from content/posts
# Usage: ./remove_backup_files.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Main execution
main() {
    echo -e "${BLUE}=== Backup File Cleanup Script ===${NC}"
    echo "This script will:"
    echo "1. Find all .md.backup files in content/posts/ recursively"
    echo "2. Show you what will be deleted"
    echo "3. Ask for confirmation before deletion"
    echo ""
    
    # Check if we're in the right directory
    if [ ! -d "content/posts" ]; then
        echo -e "${RED}Error: content/posts directory not found.${NC}"
        echo "Please run this script from the project root directory."
        exit 1
    fi
    
    # Find all .md.backup files
    echo -e "${BLUE}Searching for .md.backup files...${NC}"
    backup_files=$(find content/posts -type f -name "*.md.backup" 2>/dev/null)
    
    if [ -z "$backup_files" ]; then
        echo -e "${GREEN}No .md.backup files found in content/posts/${NC}"
        exit 0
    fi
    
    # Count and display files
    backup_count=$(echo "$backup_files" | wc -l)
    echo -e "${YELLOW}Found ${backup_count} .md.backup files:${NC}"
    echo ""
    
    # Show all files that will be deleted
    while IFS= read -r file; do
        if [ -n "$file" ]; then
            echo -e "${RED}  ${file}${NC}"
        fi
    done <<< "$backup_files"
    
    echo ""
    echo -e "${YELLOW}⚠ WARNING: These backup files will be PERMANENTLY DELETED! ⚠${NC}"
    echo -e "${YELLOW}Make sure you're satisfied with the current markdown files before proceeding.${NC}"
    echo ""
    
    # Ask for confirmation
    echo -e "${YELLOW}Are you sure you want to delete all ${backup_count} backup files? (yes/no)${NC}"
    read -r response
    
    if [[ "$response" =~ ^[Yy][Ee][Ss]$ ]]; then
        echo ""
        echo -e "${BLUE}Deleting backup files...${NC}"
        
        deleted_count=0
        while IFS= read -r file; do
            if [ -n "$file" ] && [ -f "$file" ]; then
                rm "$file"
                echo -e "${GREEN}✓ Deleted: ${file}${NC}"
                ((deleted_count++))
            fi
        done <<< "$backup_files"
        
        echo ""
        echo -e "${GREEN}=== Cleanup Complete ===${NC}"
        echo -e "${GREEN}Total backup files deleted: ${deleted_count}${NC}"
        echo ""
        echo -e "${BLUE}All .md.backup files have been removed from content/posts/${NC}"
        
    else
        echo ""
        echo -e "${BLUE}Operation cancelled. No backup files were deleted.${NC}"
        exit 0
    fi
}

# Run main function
main "$@"
