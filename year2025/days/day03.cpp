#include <iostream>
#include <sstream>
#include <string>
#include <vector>

u_int64_t compute_joltage(const std::vector<int> &digits, size_t num_iterations)
{
    size_t len = digits.size();
    u_int64_t joltage = 0;
    int max_index = -1;
    while (num_iterations-- > 0)
    {
        joltage *= 10;
        max_index++;
        int max_value = digits[max_index];
        for (size_t i = max_index; i < len - num_iterations; i++)
        {
            int c = digits[i];
            if (c > max_value)
            {
                max_value = c;
                max_index = i;
            }
        }
        joltage += max_value;
    }
    // std::cout << "Joltage for line " << line << " is " << joltage << "\n";
    return joltage;
}

int main()
{
    std::string line;

    u_int64_t part1 = 0;
    u_int64_t part2 = 0;

    while (std::getline(std::cin, line))
    {
        // Parse line into digits
        // Doing c - '0' feels like a hack but apparently is common in C++
        std::vector<int> digits;
        for (char c : line)
            digits.push_back(c - '0');

        part1 += compute_joltage(digits, 2);
        part2 += compute_joltage(digits, 12);
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
