#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <queue>


// Function to count paths from start to end using BFS of a connected directed acyclic graph.
int64_t count_paths(const std::map<std::string, std::vector<std::string>> &graph,
                    const std::map<std::string, int> &in_degree,
                    const std::string &start,
                    const std::string &end)
{
    auto remaining_degree = in_degree; // Make a copy to modify
    std::map<std::string, int64_t> path_count;
    path_count[start] = 1;
    // Since graph is a directed acyclic graph, at least one node will have in-degree 0.
    // Since the graph is connected, these nodes will be connected to the start node, 
    // so it is sufficient to initialize the queue with these.
    std::queue<std::string> queue;
    for (const auto &[node, degree] : remaining_degree)
        if (degree == 0)
            queue.push(node);
    
    while (!queue.empty())
    {
        std::string current = queue.front();
        queue.pop();

        if (current == end)
            continue;

        for (const std::string &neighbour : graph.at(current))
        {
            path_count[neighbour] += path_count[current];
            if (--remaining_degree[neighbour] == 0)
                queue.push(neighbour);
        }
    }

    return path_count[end];
}

int main()
{
    std::map<std::string, std::vector<std::string>> graph;
    std::string line;

    while (std::getline(std::cin, line))
    {
        std::stringstream ss(line);
        // Get the first node and remove colon
        std::string node;
        std::getline(ss, node, ' ');
        node = node.substr(0, node.size() - 1);
        graph[node] = {};
        // Set neighbours
        std::string neighbour;
        std::vector<std::string> neighbours;
        while (std::getline(ss, neighbour, ' '))
        {
            neighbours.push_back(neighbour);
            // Ensure neighbour exists in graph
            // Required to account for dead-end nodes
            graph[neighbour];
        }
        graph[node] = neighbours;
    }

    // In-degree map
    // C++ map initializes missing keys to zero by default so no check or initialization is needed
    std::map<std::string, int> in_degree;
    for (const auto &[node, neighbours] : graph)
    {
        in_degree[node]; // Ensure node exists in in_degree map
        for (const std::string &neighbour : neighbours)
            in_degree[neighbour]++;
    }

    // PART 1
    std::cout << "Part 1: " << count_paths(graph, in_degree, "you", "out") << "\n";

    // PART 2
    // Since the graph is acyclic, no path can visit fft after visiting dac.
    // Therefore, path counting can simply be split into three segments.
    int64_t to_fft = count_paths(graph, in_degree, "svr", "fft");
    int64_t to_dac = count_paths(graph, in_degree, "fft", "dac");
    int64_t to_out = count_paths(graph, in_degree, "dac", "out");
    std::cout << "Part 2: " << to_fft * to_dac * to_out << "\n";
    return 0;
}
