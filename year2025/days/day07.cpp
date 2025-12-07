#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <cstdint>
#include <numeric>

int main()
{
    std::string line;
    std::getline(std::cin, line);

    // Count of beam splits
    int split_count = 0;
    // Record of number of beams at each position
    std::vector<u_int64_t> beams(line.size(), 0);
    beams[line.find('S')] = 1;

    // Skip empty line
    std::getline(std::cin, line);

    while (std::getline(std::cin, line))
    {
        std::vector<u_int64_t> new_beams = beams;
        for (size_t i = 0; i < line.size(); i++)
        {
            if (line[i] == '^')
            {
                // Split the beams at this position
                new_beams[i - 1] += beams[i];
                new_beams[i + 1] += beams[i];
                new_beams[i] = 0;
                split_count++;
            }
        }

        // Update beam positions for next iteration
        beams = new_beams;

        // Skip empty line
        std::getline(std::cin, line);
    }

    // PART 1
    int part1 = split_count;
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    u_int64_t part2 = std::reduce(beams.begin(), beams.end(), uint64_t(0));
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
