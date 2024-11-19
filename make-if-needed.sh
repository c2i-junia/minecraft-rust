#!/bin/bash

# Directories to monitor
SRC_DIRS=("client/src" "server/src" "shared/src")

# File to store the last known checksum
CHECKSUM_FILE=".src_checksums"

# Initialize variables
CURRENT_CHECKSUMS=""
PREVIOUS_CHECKSUMS=""

# Generate checksums for each directory
for DIR in "${SRC_DIRS[@]}"; do
    if [ -d "$DIR" ]; then
        # Compute checksum for this directory
        DIR_CHECKSUM=$(find "$DIR" -type f -exec sha256sum {} + | sha256sum | awk '{print $1}')
        CURRENT_CHECKSUMS+="$DIR:$DIR_CHECKSUM\n"
    else
        echo "Warning: Directory $DIR does not exist."
    fi
done

# Remove trailing newline from CURRENT_CHECKSUMS
CURRENT_CHECKSUMS=$(echo -e "$CURRENT_CHECKSUMS" | sed '/^$/d')

# Check if the checksum file exists
if [ -f "$CHECKSUM_FILE" ]; then
    # Read the previous checksums
    PREVIOUS_CHECKSUMS=$(cat "$CHECKSUM_FILE")
fi

# Compare current and previous checksums
if [ "$CURRENT_CHECKSUMS" != "$PREVIOUS_CHECKSUMS" ]; then
    echo "Changes detected in one or more src directories. Running 'make'..."
    make

    # Update the stored checksum file
    echo -e "$CURRENT_CHECKSUMS" > "$CHECKSUM_FILE"
else
    echo "No changes detected in any src directory. Skipping 'make'."
fi

