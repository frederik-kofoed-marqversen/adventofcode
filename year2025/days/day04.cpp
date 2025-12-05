#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <optional>

bool is_removable(const std::vector<std::vector<char>> &data, size_t i, size_t j)
{
    const char PAPER_ROLL = '@';
    int neighbours = 0;
    if (data[i][j] != PAPER_ROLL)
        return false;
    for (int di : {-1, 0, 1})
    {
        for (int dj : {-1, 0, 1})
        {
            char c = data[i + di][j + dj];
            if (di == 0 && dj == 0)
                continue;
            if (c == PAPER_ROLL)
                neighbours++;
        }
    }
    return neighbours < 4;
}

std::optional<std::pair<size_t, size_t>> find_removable(const std::vector<std::vector<char>> &data)
{
    for (size_t i = 1; i < data.size() - 1; i++)
        for (size_t j = 1; j < data[0].size() - 1; j++)
            if (is_removable(data, i, j))
                return std::make_pair(i, j);
    return std::nullopt;
}

int main()
{
    std::string line;
    std::vector<std::vector<char>> data;

    // Add padding to avoid boundary checks
    while (std::getline(std::cin, line))
    {
        std::vector<char> row;
        row.push_back('.');
        for (char c : line)
            row.push_back(c);
        row.push_back('.');
        data.push_back(row);
    }
    data.insert(data.begin(), std::vector<char>(data[0].size(), '.'));
    data.push_back(std::vector<char>(data[0].size(), '.'));

    // PART 1
    int part1 = 0;
    for (size_t i = 1; i < data.size() - 1; i++)
        for (size_t j = 1; j < data[0].size() - 1; j++)
            if (is_removable(data, i, j))
                part1++;

    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    int part2 = 0;
    while (auto pos = find_removable(data))
    {
        auto [i, j] = *pos;
        data[i][j] = '.';
        part2++;
    }
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
