#!/bin/bash

# Check if day number is provided
if [ -z "$1" ]; then
    echo "Usage: ./setup_day.sh NN"
    exit 1
fi

DAY=$(printf "%02d" $1)

# Create the day source file
DAY_FILE="days/day${DAY}.cpp"
if [ -f "$DAY_FILE" ]; then
    echo "Error: $DAY_FILE already exists!"
    exit 1
fi

# Create the solution template
cat > "$DAY_FILE" << 'EOF'
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main()
{
    std::string line;
    
    while (std::getline(std::cin, line))
    {
        // Parse input
    }

    // PART 1
    int part1 = 0;
    std::cout << "Part 1: " << part1 << "\n";
    
    // PART 2
    int part2 = 0;
    std::cout << "Part 2: " << part2 << "\n";
    
    return 0;
}
EOF

# Create placeholder input files in a single folder
touch "input/day${DAY}.txt"
touch "input/day${DAY}_test.txt"

echo "Created:"
echo "  - $DAY_FILE"
echo "  - input/day${DAY}.txt"
echo "  - input/day${DAY}_test.txt"
echo ""
echo "To build: make"
echo "To run:   make run DAY=${DAY}"
echo "To test:  make run DAY=${DAY} TEST=1"
