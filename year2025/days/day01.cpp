#include <sstream>
#include <string>
#include <iostream>
#include <vector>

int main()
{
    std::vector<std::pair<char, int>> data;
    std::string line;

    while (std::getline(std::cin, line))
    {
        std::stringstream ss(line);
        char cmd;
        int val;
        ss >> cmd >> val;
        data.push_back({cmd, val});
    }

    int dial = 50;
    int part1 = 0;
    int part2 = 0;

    for (const auto &[cmd, val] : data)
    {
        int sign = (cmd == 'R') ? 1 : -1;
        int previous_dial = dial;

        int move = val;
        while (move-- > 0)
        {
            dial += sign;
            dial %= 100;

            if (dial == 0)
                part2++;
        }

        dial %= 100;

        if (dial == 0)
            part1++;
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";
    // PART 2
    std::cout << "Part 2: " << part2 << "\n";
    return 0;
}