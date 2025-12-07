#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <cstdint>
#include <numeric>

int main()
{
    std::string line;
    
    // Read initial line
    std::getline(std::cin, line);

    int split_count = 0;
    // Record the of number of beams at each position
    std::vector<u_int64_t> beams(line.size(), 0);
    beams[line.find('S')] = 1;

    // Skip empty line
    std::getline(std::cin, line);

    while (std::getline(std::cin, line))
    {
        for (size_t i = 0; i < line.size(); i++)
        {
            if (line[i] == '^')
            {
                // Increment split_count only if beams are present
                split_count += beams[i] > 0 ? 1 : 0;
                // Split the beams at this position
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }

        // Skip empty line
        std::getline(std::cin, line);
    }

    // PART 1
    std::cout << "Part 1: " << split_count << "\n";

    // PART 2
    u_int64_t part2 = std::reduce(beams.begin(), beams.end(), uint64_t(0));
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
