#!/bin/bash

# Script to convert HEIC images to WebP and update markdown references
# Usage: ./convert_heic_to_webp.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if ImageMagick is installed
if ! command -v magick &> /dev/null; then
    echo -e "${RED}Error: ImageMagick is not installed.${NC}"
    echo "Install it with: brew install imagemagick"
    exit 1
fi

# Function to convert HEIC to WebP
convert_heic_to_webp() {
    local heic_file="$1"
    local webp_file="${heic_file%.heic}.webp"
    
    echo -e "${BLUE}Converting: ${heic_file}${NC}"
    
    # Convert HEIC to WebP with good quality and compression
    magick "$heic_file" -quality 85 "$webp_file"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Converted to: ${webp_file}${NC}"
        
        # Get file sizes for comparison
        heic_size=$(du -h "$heic_file" | cut -f1)
        webp_size=$(du -h "$webp_file" | cut -f1)
        echo -e "${YELLOW}  Size: ${heic_size} → ${webp_size}${NC}"
        
        return 0
    else
        echo -e "${RED}✗ Failed to convert: ${heic_file}${NC}"
        return 1
    fi
}

# Function to update markdown references
update_markdown_references() {
    local heic_file="$1"
    local webp_file="${heic_file%.heic}.webp"
    local dir=$(dirname "$heic_file")
    local index_file="$dir/index.md"
    
    if [ -f "$index_file" ]; then
        echo -e "${BLUE}Updating references in: ${index_file}${NC}"
        
        # Get filenames for replacement
        heic_basename=$(basename "$heic_file")
        webp_basename=$(basename "$webp_file")
        
        # Create backup
        cp "$index_file" "${index_file}.backup"
        
        # Replace references in markdown
        # Handle various markdown image syntaxes including Substack URLs
        sed -i.tmp \
            -e "s|${heic_basename}|${webp_basename}|g" \
            -e "s|https://substackcdn.com/image/fetch/[^)]*${heic_basename%.heic}[^)]*|./${webp_basename}|g" \
            -e "s|https://substackcdn.com/image/fetch/[^)]*${heic_basename}[^)]*|./${webp_basename}|g" \
            "$index_file"
        
        # Remove temporary file
        rm -f "${index_file}.tmp"
        
        echo -e "${GREEN}✓ Updated references in: ${index_file}${NC}"
    else
        echo -e "${YELLOW}⚠ No index.md found in: ${dir}${NC}"
    fi
}

# Function to process a single directory
process_directory() {
    local dir="$1"
    local converted_count=0
    local updated_count=0
    
    # Initialize array to store HEIC files for later removal
    if [ -z "${heic_files_to_remove+x}" ]; then
        declare -a heic_files_to_remove
    fi
    
    echo -e "${BLUE}Processing directory: ${dir}${NC}"
    
    # Find all HEIC files in this directory
    while IFS= read -r -d '' heic_file; do
        if [ -f "$heic_file" ]; then
            # Convert HEIC to WebP
            if convert_heic_to_webp "$heic_file"; then
                ((converted_count++))
                
                # Update markdown references
                if update_markdown_references "$heic_file"; then
                    ((updated_count++))
                fi
                
                # Store HEIC file for later removal (user will choose at the end)
                heic_files_to_remove+=("$heic_file")
            fi
        fi
    done < <(find "$dir" -maxdepth 1 -name "*.heic" -print0)
    
    echo -e "${GREEN}Directory ${dir}: ${converted_count} images converted, ${updated_count} markdown files updated${NC}"
}

# Main execution
main() {
    echo -e "${BLUE}=== HEIC to WebP Converter (January Test) ===${NC}"
    echo "This script will:"
    echo "1. Find all HEIC images in content/posts/2025/01/"
    echo "2. Convert them to WebP format"
    echo "3. Update markdown references"
    echo "4. Optionally remove original HEIC files"
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
    
    # Find all directories containing HEIC files in January only
    heic_dirs=$(find content/posts/2025/01 -type f -name "*.heic" -exec dirname {} \; | sort -u)
    
    if [ -z "$heic_dirs" ]; then
        echo -e "${YELLOW}No HEIC files found in content/posts/2025/01/${NC}"
        exit 0
    fi
    
    heic_count=$(echo "$heic_dirs" | wc -l)
    echo -e "${GREEN}Found HEIC files in ${heic_count} directories in January${NC}"
    echo ""
    
    # Process each directory
    total_converted=0
    total_updated=0
    
    while IFS= read -r dir; do
        if [ -n "$dir" ]; then
            process_directory "$dir"
            echo ""
        fi
    done <<< "$heic_dirs"
    
    echo -e "${GREEN}=== January Conversion Complete ===${NC}"
    echo -e "${GREEN}Total images converted: ${total_converted}${NC}"
    echo -e "${GREEN}Total markdown files updated: ${total_updated}${NC}"
    echo ""
    
    # Ask about removing original HEIC files
    if [ ${#heic_files_to_remove[@]} -gt 0 ]; then
        echo -e "${YELLOW}Found ${#heic_files_to_remove[@]} HEIC files that were converted.${NC}"
        echo -e "${YELLOW}Remove all original HEIC files? (y/n)${NC}"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            for heic_file in "${heic_files_to_remove[@]}"; do
                if [ -f "$heic_file" ]; then
                    rm "$heic_file"
                    echo -e "${GREEN}✓ Removed: ${heic_file}${NC}"
                fi
            done
        else
            echo -e "${BLUE}Original HEIC files were preserved.${NC}"
        fi
    fi
    
    echo ""
    echo -e "${YELLOW}Note: Backup files (.backup) were created for all modified markdown files.${NC}"
    echo -e "${BLUE}This was a test run on January folder only.${NC}"
}

# Run main function
main "$@"
