#!/bin/bash

# Check if a day number is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

# Ensure provided day number is an integer
if ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: Day number must be an integer."
    exit 1
fi

# Get provided day number
DAY=$1
# Format the day number as two digits (e.g., 01, 02, ...)
DAY_PADDED=$(printf "%02d" $DAY)

# Compute absolute paths
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
DIR="$SCRIPT_DIR/src/days"
FILE_PATH="$DIR/day${DAY_PADDED}.rs"

# Check if the file already exists
if [ -e "$FILE_PATH" ]; then
    echo "Error: File $FILE_PATH already exists."
    exit 1
fi

# Create the file with the desired contents
mkdir -p "$DIR" # Ensure the target directory exists
cat <<EOL > "$FILE_PATH"
pub fn run(use_test_input: bool) {
    let input = super::read_input($DAY, use_test_input);

    // PART 1
    println!("Result part 1: {}", 0);
    // PART 2
    println!("Result part 2: {}", 0);
}
EOL

echo "Created $FILE_PATH successfully."