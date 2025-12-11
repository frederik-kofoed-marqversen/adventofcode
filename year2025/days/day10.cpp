#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <queue>
#include <map>
#include <optional>
#include <limits>
#include <algorithm>

// BFS to find the minimum number of button presses to reach the target light configuration
int num_button_presses(
    const std::vector<std::vector<int>> &buttons,
    std::vector<int> target)
{
    std::map<std::vector<int>, int> visited;
    std::queue<std::pair<std::vector<int>, int>> q;

    std::vector<int> start(target.size(), 0);
    q.push({start, 0});
    visited.insert({start, -1});

    while (!q.empty())
    {
        auto [current, presses] = q.front();
        q.pop();

        for (size_t i = 0; i < buttons.size(); ++i)
        {
            const auto &button = buttons[i];
            std::vector<int> next = current;
            for (size_t j : button)
                next[j] ^= 1;

            if (visited.find(next) != visited.end())
                continue;

            visited.insert({next, i});
            q.push({next, presses + 1});

            if (next == target)
            {
                return presses + 1;
                std::vector<int> result;
                // // Reconstruct the sequence of button presses
                // while (visited[next] != -1)
                // {
                //     int button = visited[next];
                //     result.push_back(button);
                //     for (size_t j : buttons[button])
                //         next[j] ^= 1;
                // }
                // return result;
            }
        }
    }

    return -1;
}

std::vector<int> get_lights(const std::string &line)
{
    size_t start = line.find('[');
    size_t end = line.find(']');
    std::string content = line.substr(start + 1, end - start - 1);

    std::vector<int> lights;
    for (char c : content)
        lights.push_back(c == '#' ? 1 : 0);
    return lights;
}

std::vector<std::vector<int>> get_buttons(const std::string &line)
{
    std::vector<std::vector<int>> buttons;
    size_t start = 0;
    while ((start = line.find('(', start)) != std::string::npos)
    {
        size_t end = line.find(')', start);
        std::string content = line.substr(start + 1, end - start - 1);
        std::vector<int> button_values;
        std::stringstream ss(content);
        std::string value;
        while (std::getline(ss, value, ','))
            button_values.push_back(std::stoi(value));
        buttons.push_back(button_values);
        start = end + 1;
    }
    return buttons;
}

std::vector<int> get_joltages(const std::string &line)
{
    size_t start = line.find('{');
    size_t end = line.find('}');
    std::string content = line.substr(start + 1, end - start - 1);

    std::vector<int> joltages;
    std::stringstream ss(content);
    std::string value;
    while (std::getline(ss, value, ','))
        joltages.push_back(std::stoi(value));
    return joltages;
}

int main()
{
    int part1 = 0;
    int part2 = 0;

    std::string line;

    while (std::getline(std::cin, line))
    {
        auto lights = get_lights(line);
        auto buttons = get_buttons(line);
        auto joltages = get_joltages(line);

        part1 += num_button_presses(buttons, lights);
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
