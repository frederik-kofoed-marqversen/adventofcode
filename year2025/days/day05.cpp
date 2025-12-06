#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <cstdint>

void add_range(std::vector<std::pair<uint64_t, uint64_t>> &ranges, uint64_t a, uint64_t b)
{
    // Find first range that starts at or after 'a' using
    // lower_bound(value) which finds the first element >= value.
    // Method returns an iterator to the element
    // The method returns the insertion point for the new range [a, b] to keep the list sorted
    // Default comparison for pair compares first element, then second element
    auto it = std::lower_bound(ranges.begin(), ranges.end(), std::make_pair(a, uint64_t(0)));

    // If a previous range exists, check for overlap
    if (it != ranges.begin())
    {
        auto prev = it - 1;
        if (prev->second >= a - 1)
        {
            a = prev->first;
            b = std::max(b, prev->second);
            it = ranges.erase(prev);
        }
    }

    // All ranges that start before the end of the new range [a, b] has overlap and thus need merging
    while (it != ranges.end() && it->first <= b + 1)
    {
        b = std::max(b, it->second);
        it = ranges.erase(it);
    }

    // Insert the nonoverlapping range [a, b] at its sorted position
    ranges.insert(it, {a, b});
}

int main()
{
    std::string line;
    std::vector<std::pair<uint64_t, uint64_t>> data; // data holds a sorted list of non-overlapping ranges

    while (std::getline(std::cin, line) && !line.empty())
    {
        uint64_t a, b;
        char dash;
        std::istringstream ss(line);
        ss >> a >> dash >> b;
        add_range(data, a, b);
    }

    // PART 1
    int part1 = 0;
    while (std::getline(std::cin, line))
    {
        uint64_t id = std::stoul(line);
        // Check if id is in any range
        for (const auto &range : data)
        {
            if (id >= range.first && id <= range.second)
            {
                part1++;
                break;
            }
        }
    }
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    uint64_t part2 = 0;
    for (auto &range : data)
        part2 += (range.second - range.first + 1);
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
