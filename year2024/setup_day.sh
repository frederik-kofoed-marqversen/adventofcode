#!/bin/bash

# Check if day number is provided
if [ -z "$1" ]; then
    echo "Usage: ./setup_day.sh NN"
    echo "Example: ./setup_day.sh 03"
    exit 1
fi

DAY=$(printf "%02d" $1)

# Create the day source file
DAY_FILE="src/days/day${DAY}.rs"
if [ -f "$DAY_FILE" ]; then
    echo "Error: $DAY_FILE already exists!"
    exit 1
fi

# Create the solution template
cat > "$DAY_FILE" << 'EOF'
pub fn run(use_test_input: bool) {
    let input = super::read_input($DAY, use_test_input);

    // PART 1
    println!("Result part 1: {}", 0);
    // PART 2
    println!("Result part 2: {}", 0);
}
EOF

# Replace $DAY placeholder with actual day number
sed -i "s/\$DAY/$1/g" "$DAY_FILE"

# Create placeholder input files
touch "input/day${DAY}.txt"
touch "input/day${DAY}_test.txt"

# Update mod.rs
MODFILE="src/days/mod.rs"
if [ -f "$MODFILE" ]; then
    if ! grep -q "pub mod day${DAY};" "$MODFILE"; then
        sed -i "1i\pub mod day${DAY};" "$MODFILE"
        echo "Updated $MODFILE"
    # Update main.rs
    MAINFILE="src/main.rs"
    if [ -f "$MAINFILE" ]; then
        if ! grep -q "day_map.insert(${DAY#0}," "$MAINFILE"; then
            sed -i "/let mut day_map: HashMap<u32, fn(bool)> = HashMap::new();/a \    day_map.insert(${DAY#0}, days::day${DAY}::run);" "$MAINFILE"
            echo "Updated $MAINFILE"
        fi
    fi

    fi
fi

echo "Created:"
echo "  - $DAY_FILE"
echo "  - input/day${DAY}.txt"
echo "  - input/day${DAY}_test.txt"
echo ""

# Update mod.rs (register module)
MODFILE="src/days/mod.rs"
if [ -f "$MODFILE" ]; then
    if ! grep -q "pub mod day${DAY};" "$MODFILE"; then
        sed -i "1i\pub mod day${DAY};" "$MODFILE"
        echo "Updated module registration in $MODFILE"
    fi
fi

# Update main.rs (register runner)
MAINFILE="src/main.rs"
if [ -f "$MAINFILE" ]; then
    if ! grep -q "day_map.insert(${DAY#0}," "$MAINFILE"; then
        sed -i "/let mut day_map: HashMap<u32, fn(bool)> = HashMap::new();/a \    day_map.insert(${DAY#0}, days::day${DAY}::run);" "$MAINFILE"
        echo "Registered day ${DAY#0} runner in $MAINFILE"
    fi
fi