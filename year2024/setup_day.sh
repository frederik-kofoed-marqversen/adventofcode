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

# Get provided day number and format as two digit number (e.g., 01, 02, ...)
DAY=$(printf "%02d" $1)

# Compute absolute paths
HOME=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
DIR="$HOME/src/days"
FILE_PATH="$DIR/day${DAY}.rs"

FILENAME="day${DAY}.rs"
FILE="$HOME/src/days/$FILENAME"
MODFILE="$HOME/src/days/mod.rs"
MAINFILE="$HOME/src/main.rs"

# Check if the file already exists
if [ -e "$FILE" ]; then
    echo "Error: File $FILE already exists."
    exit 1
fi

# Create the file with default contents
mkdir -p "$HOME/src/days" # Ensure the target directory exists
cat <<EOL > "$FILE"
pub fn run(use_test_input: bool) {
    let input = super::read_input($DAY, use_test_input);

    // PART 1
    println!("Result part 1: {}", 0);
    // PART 2
    println!("Result part 2: {}", 0);
}
EOL

echo "Created $FILE_PATH successfully."

# Update mod.rs
if [ -f "$MODFILE" ]; then
    if ! grep -q "pub mod day${DAY};" "$MODFILE"; then
        sed -i "1i\pub mod day${DAY};" "$MODFILE"
        echo "Updated $MODFILE"
    else
        echo "Warning: days/mod.rs already contains day${DAY};"
    fi
else
    echo "$MODFILE not found"
    exit 1
fi

# Update main.rs
if [ -f "$MAINFILE" ]; then
    if ! grep -q "day_map.insert(${DAY}," "$MAINFILE"; then
        sed -i "/let mut day_map: HashMap<u32, fn(bool)> = HashMap::new();/a \ \ \ \ day_map.insert(${DAY}, days::day${DAY}::run);" "$MAINFILE"
        echo "Updated $MAINFILE"
    else
        echo "Warning: main.rs already includes day${DAY}"
    fi
else
    echo "$MAINFILE not found"
    exit 1
fi