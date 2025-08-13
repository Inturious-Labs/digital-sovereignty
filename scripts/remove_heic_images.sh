#!/bin/bash

# Script to remove all HEIC images recursively from content/posts
# Usage: ./remove_heic_images.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Main execution
main() {
    echo -e "${BLUE}=== HEIC Image Removal Script ===${NC}"
    echo "This script will:"
    echo "1. Find all HEIC images in content/posts/ recursively"
    echo "2. Show you what will be deleted"
    echo "3. Ask for confirmation before deletion"
    echo ""
    
    # Check if we're in the right directory
    if [ ! -d "content/posts" ]; then
        echo -e "${RED}Error: content/posts directory not found.${NC}"
        echo "Please run this script from the project root directory."
        exit 1
    fi
    
    # Find all HEIC files
    echo -e "${BLUE}Searching for HEIC files...${NC}"
    heic_files=$(find content/posts -type f -name "*.heic" 2>/dev/null)
    
    if [ -z "$heic_files" ]; then
        echo -e "${GREEN}No HEIC files found in content/posts/${NC}"
        exit 0
    fi
    
    # Count and display files
    heic_count=$(echo "$heic_files" | wc -l)
    echo -e "${YELLOW}Found ${heic_count} HEIC files:${NC}"
    echo ""
    
    # Show all files that will be deleted
    while IFS= read -r file; do
        if [ -n "$file" ]; then
            echo -e "${RED}  ${file}${NC}"
        fi
    done <<< "$heic_files"
    
    echo ""
    echo -e "${YELLOW}⚠ WARNING: These files will be PERMANENTLY DELETED! ⚠${NC}"
    echo ""
    
    # Ask for confirmation
    echo -e "${YELLOW}Are you sure you want to delete all ${heic_count} HEIC files? (yes/no)${NC}"
    read -r response
    
    if [[ "$response" =~ ^[Yy][Ee][Ss]$ ]]; then
        echo ""
        echo -e "${BLUE}Deleting HEIC files...${NC}"
        
        deleted_count=0
        while IFS= read -r file; do
            if [ -n "$file" ] && [ -f "$file" ]; then
                rm "$file"
                echo -e "${GREEN}✓ Deleted: ${file}${NC}"
                ((deleted_count++))
            fi
        done <<< "$heic_files"
        
        echo ""
        echo -e "${GREEN}=== Deletion Complete ===${NC}"
        echo -e "${GREEN}Total HEIC files deleted: ${deleted_count}${NC}"
        echo ""
        echo -e "${BLUE}All HEIC images have been removed from content/posts/${NC}"
        
    else
        echo ""
        echo -e "${BLUE}Operation cancelled. No files were deleted.${NC}"
        exit 0
    fi
}

# Run main function
main "$@"
