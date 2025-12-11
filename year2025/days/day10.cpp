#include <bitset>
#include <iostream>
#include <queue>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>
#include <z3++.h>

constexpr size_t MAX_SIZE = 64;
using BitArr = std::bitset<MAX_SIZE>; // All zeros on initialization

// BFS to find the minimum number of button presses to reach the target light configuration
int solve_lights_bfs(const std::vector<BitArr> &buttons, const BitArr &target)
{
    std::unordered_set<BitArr> visited;
    std::queue<std::pair<BitArr, int>> q;

    BitArr start;
    q.push({start, 0});
    visited.insert(start);

    while (!q.empty())
    {
        auto [current, presses] = q.front();
        q.pop();

        for (size_t i = 0; i < buttons.size(); ++i)
        {
            BitArr next = current ^ buttons[i];

            if (visited.find(next) != visited.end())
                continue;

            visited.insert({next, i});
            q.push({next, presses + 1});

            if (next == target)
                return presses + 1;
        }
    }

    return -1;
}

// Solve Ax = b with x >= 0, minimising sum(x) using Z3
// Takes A in column-major format (A[i] is the i-th column)
int solve_joltage_z3(
    const std::vector<BitArr> &A,
    const std::vector<int> &b)
{
    size_t num_col = A.size();
    size_t num_rows = b.size();

    z3::context c;
    z3::optimize opt(c);

    // Create variables x_i
    std::vector<z3::expr> x;
    for (size_t i = 0; i < num_col; ++i)
    {
        std::stringstream x_name;
        x_name << "x_" << i;
        x.push_back(c.int_const(x_name.str().c_str()));

        // Add non-negativity constraint
        opt.add(x[i] >= 0);
    }

    // Add constraints: Ax = b
    for (size_t j = 0; j < num_rows; ++j)
    {
        z3::expr sum = c.int_val(0);
        for (size_t i = 0; i < num_col; ++i)
            // Check if button i affects position j using bitset test
            if (A[i].test(j))
                sum = sum + x[i];
        opt.add(sum == c.int_val(b[j]));
    }

    // Objective: minimise sum(x)
    z3::expr objective = c.int_val(0);
    for (size_t i = 0; i < num_col; ++i)
        objective = objective + x[i];
    opt.minimize(objective);

    // Solve and return minimised objective value
    if (opt.check() == z3::sat)
        return opt.get_model().eval(objective).get_numeral_int();
    else
        return -1;
}

BitArr get_lights(const std::string &line)
{
    size_t start = line.find('[');
    size_t end = line.find(']');
    std::string content = line.substr(start + 1, end - start - 1);

    BitArr lights;
    for (size_t i = 0; i < content.size(); ++i)
        if (content[i] == '#')
            lights.set(i);
    return lights;
}

std::vector<BitArr> get_buttons(const std::string &line)
{
    std::vector<BitArr> buttons;

    size_t start = 0;
    while ((start = line.find('(', start)) != std::string::npos)
    {
        size_t end = line.find(')', start);
        std::string content = line.substr(start + 1, end - start - 1);
        std::stringstream ss(content);

        BitArr button;
        std::string value;
        while (std::getline(ss, value, ','))
            button.set(std::stoi(value));

        buttons.push_back(button);
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

        part1 += solve_lights_bfs(buttons, lights);

        auto result = solve_joltage_z3(buttons, joltages);
        if (result == -1)
            std::cerr << "Warning: Non-satisfiable problem encountered\n";
        part2 += result;
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
