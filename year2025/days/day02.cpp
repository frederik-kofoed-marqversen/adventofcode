#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <numeric>
#include <cstdint>

std::pair<bool, size_t> is_repetition(uint64_t n)
{
    const std::vector<size_t> primes = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97};

    std::string n_str = std::to_string(n);
    size_t len = n_str.length();

    // Check for each prime if string consist of prime repetitions
    for (size_t num_repeats : primes)
    {
        // Skip if length is not divisible by num_repeats
        if (len % num_repeats != 0)
            continue;

        size_t sub_len = len / num_repeats;
        std::string pattern = n_str.substr(0, sub_len);

        // Check if each substring matches pattern
        bool is_repetition = true;
        for (size_t i = 1; i < num_repeats; i++)
        {
            if (n_str.substr(i * sub_len, sub_len) != pattern)
            {
                is_repetition = false;
                break;
            }
        }

        if (is_repetition)
            return {true, num_repeats};
    }

    return {false, 0};
}

int main()
{
    // Read input
    std::vector<std::pair<uint64_t, uint64_t>> data;
    std::string line;

    // Parse input
    while (std::getline(std::cin, line, ','))
    {
        std::string first, second;
        std::stringstream stream(line);

        uint64_t a, b;
        char dash;
        stream >> a >> dash >> b; // Reads full uinteger regardless of length

        data.push_back({a, b});
    }

    // Solve parts
    uint64_t part1 = 0;
    uint64_t part2 = 0;

    for (const auto &[a, b] : data)
    {
        for (uint64_t n = a; n <= b; n++)
        {
            const auto &[is_invalid, repeat_count] = is_repetition(n);
            if (!is_invalid)
                continue;

            part2 += n;
            if (repeat_count == 2)
                part1 += n;
        }
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";
    // PART 2
    std::cout << "Part 2: " << part2 << "\n";
    return 0;
}
