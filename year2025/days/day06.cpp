#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <cstdint>
#include <numeric>

std::pair<std::vector<std::vector<std::vector<char>>>, std::vector<char>> parse_input(std::vector<std::string> &lines)
{
    std::vector<std::vector<std::vector<char>>> data;
    std::vector<char> operations;

    // Last line contains operations
    std::string operation_line = lines.back();
    lines.pop_back();

    // Parse problems
    std::vector<std::vector<char>> problem(lines.size());
    operations.push_back(operation_line[0]);
    for (size_t i = 0; i < lines[0].size(); i++)
    {
        // Check if new problem starts
        char op = operation_line[i];
        if (op != ' ' && i != 0)
        {
            // Remove empty column from previous problem
            for (auto &row : problem)
                row.pop_back();
            // Store previous problem and start new one
            data.push_back(problem);
            operations.push_back(op);
            problem = std::vector<std::vector<char>>(lines.size());
        }
        // Add current column to current problem
        for (size_t j = 0; j < lines.size(); j++)
            problem[j].push_back(lines[j][i]);
    }
    // Add last problem
    data.push_back(problem);

    return {data, operations};
}

std::vector<uint64_t> parse_row_numbers(std::vector<std::vector<char>> &problem)
{
    size_t num_rows = problem.size();
    size_t num_cols = problem[0].size();
    std::vector<uint64_t> numbers;
    for (size_t i = 0; i < num_rows; i++)
    {
        std::string row;
        for (size_t j = 0; j < num_cols; j++)
            row += problem[i][j];
        uint64_t number = std::stoull(row);
        numbers.push_back(number);
    }
    return numbers;
}

std::vector<uint64_t> parse_column_numbers(std::vector<std::vector<char>> &problem)
{
    size_t num_rows = problem.size();
    size_t num_cols = problem[0].size();
    std::vector<uint64_t> numbers;
    for (size_t j = 0; j < num_cols; j++)
    {
        std::string column;
        for (size_t i = 0; i < num_rows; i++)
            column += problem[i][j];
        uint64_t number = std::stoull(column);
        numbers.push_back(number);
    }
    return numbers;
}

uint64_t solve_problem(std::vector<std::vector<char>> &problem, char operation, bool part2)
{
    std::vector<uint64_t> numbers;
    if (part2)
        numbers = parse_column_numbers(problem);
    else
        numbers = parse_row_numbers(problem);

    if (operation == '+')
    {
        return std::reduce(numbers.begin(), numbers.end(), uint64_t(0));
    }
    else if (operation == '*')
    {
        return std::reduce(numbers.begin(), numbers.end(), uint64_t(1), std::multiplies<uint64_t>());
    }
    else
    {
        throw std::invalid_argument("Unknown operation");
    }
}

int main()
{
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(std::cin, line))
        lines.push_back(line);
    auto [data, operations] = parse_input(lines);

    uint64_t part1 = 0;
    uint64_t part2 = 0;

    // Solve each problem
    for (size_t i = 0; i < data.size(); i++)
    {
        auto problem = data[i];
        auto operation = operations[i];

        part1 += solve_problem(problem, operation, false);
        part2 += solve_problem(problem, operation, true);
    }

    // Print results
    std::cout << "Part 1: " << part1 << "\n";
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
