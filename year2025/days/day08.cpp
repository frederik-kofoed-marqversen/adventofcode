#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <cstdint>
#include <array>
#include <numeric>
#include <algorithm>

int64_t distance_squared(const std::array<int64_t, 3> &a, const std::array<int64_t, 3> &b)
{
    int64_t dx = a[0] - b[0];
    int64_t dy = a[1] - b[1];
    int64_t dz = a[2] - b[2];
    return dx * dx + dy * dy + dz * dz;
}

// Disjoint Set Union (Union-Find) structure
// Each element points to its parent. Roots point to themselves.
// Tree structure is flattened each time find is called (path compression).
// Size array keeps track of the size of each component stored at the roots.
struct DSU
{
    std::vector<int64_t> parent, sz;
    DSU(size_t n) : parent(n), sz(n, 1) { std::iota(parent.begin(), parent.end(), 0); }
    // Find root of the set containing v with path compression
    int64_t find(int64_t v) { return parent[v] == v ? v : parent[v] = find(parent[v]); }
    // Union sets containing a and b, return true if merged, false if already in the same set
    bool unite(int64_t a, int64_t b)
    {
        // Get roots of the sets containing a and b
        a = find(a);
        b = find(b);
        if (a == b)
            return false;
        // Union by size: attach smaller tree under the root of the larger tree and update size
        if (sz[a] < sz[b])
            std::swap(a, b);
        parent[b] = a;
        sz[a] += sz[b];
        return true;
    }
    int64_t size(int64_t v) { return sz[find(v)]; }
};

int main()
{
    std::vector<std::array<int64_t, 3>> vertices;
    std::vector<std::pair<int64_t, std::pair<int64_t, int64_t>>> edges;

    // Read input vertices and create all edges
    std::string line;
    while (std::getline(std::cin, line))
    {
        std::stringstream ss(line);
        int64_t x, y, z;
        char comma;
        ss >> x >> comma >> y >> comma >> z;
        vertices.push_back({x, y, z});

        const int64_t i = vertices.size() - 1;
        const auto &vi = vertices[i];
        for (int64_t j = 0; j < i; j++)
        {
            const auto &vj = vertices[j];
            auto entry = std::make_pair(distance_squared(vi, vj), std::make_pair(j, i));
            edges.push_back(entry);
        }
    }
    // Sort edges by distance ascending
    std::sort(edges.begin(), edges.end());

    int64_t part1 = -1;
    int64_t part2 = -1;

    // Use Disjoint Set Union to process edges
    DSU dsu(vertices.size());
    int64_t edges_used = 0;
    for (auto const &entry : edges)
    {
        auto const [u, v] = entry.second;
        dsu.unite(u, v);
        edges_used++;

        if (edges_used == 1000) // Should be set to 10 for test input
        {
            // Collect sizes of all roots
            std::vector<int64_t> comps;
            for (int64_t i = 0; i < (int64_t)vertices.size(); ++i)
                if (dsu.find(i) == i)
                    comps.push_back(dsu.sz[i]);
            // Sort sizes descending and take the product of the three largest
            std::sort(comps.rbegin(), comps.rend());
            part1 = comps[0] * comps[1] * comps[2];
        }

        if (dsu.size(u) == (int64_t)vertices.size())
        {
            // All vertices are connected
            // [u, v] is the last edge that connected everything
            part2 = vertices[u][0] * vertices[v][0];
            break;
        }
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
