#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <algorithm>

enum Result
{
    SOLVABLE,
    IMPOSSIBLE,
    UNDECIDED
};

Result check_problem(std::string line, const std::vector<int> &present_sizes)
{
    int h, w, count;
    char c;
    std::stringstream ss(line);
    ss >> h >> c >> w >> c;

    int box_count = (h / 3) * (w / 3);

    int total_count = 0;
    int min_area = 0;
    for (size_t i = 0; i < present_sizes.size(); i++)
    {
        ss >> count;
        total_count += count;
        min_area += present_sizes[i] * count;
    }

    if (min_area > h * w)
        return IMPOSSIBLE;
    else if (total_count <= box_count)
        return SOLVABLE;
    else
        return UNDECIDED;
}

int main()
{
    std::vector<int> present_sizes;

    // Read 6 presents
    std::string line;
    for (int i = 0; i < 6; ++i)
    {
        std::getline(std::cin, line); // Read "n:"
        int size = 0;
        for (int j = 0; j < 3; ++j)
        {
            std::getline(std::cin, line);
            size += std::count(line.begin(), line.end(), '#');
        }
        present_sizes.push_back(size);
        std::getline(std::cin, line); // Read empty line
    }

    // Read and solve problems
    int solvable = 0, impossible = 0, undecided = 0;

    while (std::getline(std::cin, line))
    {
        switch (check_problem(line, present_sizes))
        {
        case SOLVABLE: solvable++; break;
        case IMPOSSIBLE: impossible++; break;
        case UNDECIDED: undecided++; break;
        }
    }

    // PART 1
    if (undecided > 0)
        std::cerr << "Warning: There are " << undecided << " undecided cases!\n";
    std::cout << "Part 1: " << solvable << "\n";

    // // PART 2
    // int part2 = 0;
    // std::cout << "Part 2: " << part2 << "\n";

    return 0;
}