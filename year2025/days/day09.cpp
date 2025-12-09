#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// Check if the axis-aligned bounding box (AABB) defined by two polygon vertices
// is unobstructed by polygon edges. Returns false if any edge crosses through
// the interior of the AABB (boundaries excluded).
// This is a sufficient check for the stated problem under the assumption that all polygon
// edges separate inside from outside areas (no adjacent parallel edges with zero spacing).
bool is_valid(const std::vector<std::pair<int64_t, int64_t>> &polygon, size_t idx1, size_t idx2)
{
    // AABB (rectangle) diagonal
    auto [x1, y1] = polygon[idx1];
    auto [x2, y2] = polygon[idx2];

    // AABB bounds (boundaries not included)
    int64_t xmin = std::min(x1, x2);
    int64_t xmax = std::max(x1, x2);
    int64_t ymin = std::min(y1, y2);
    int64_t ymax = std::max(y1, y2);

    // Check if any edge of the polygon intersects the interior of the AABB
    for (size_t i = 0; i < polygon.size(); i++)
    {
        auto [px1, py1] = polygon[i];
        auto [px2, py2] = polygon[(i + 1) % polygon.size()];

        if (px1 == px2) // Vertical edge at x = px1
        {
            int64_t edge_ymin = std::min(py1, py2);
            int64_t edge_ymax = std::max(py1, py2);

            if (xmin < px1 && px1 < xmax &&
                edge_ymin < ymax && ymin < edge_ymax)
                return false;
        }
        else if (py1 == py2) // Horisontal edge at y = py1
        {
            int64_t edge_xmin = std::min(px1, px2);
            int64_t edge_xmax = std::max(px1, px2);

            if (ymin < py1 && py1 < ymax &&
                edge_xmin < xmax && xmin < edge_xmax)
                return false;
        }
    }
    return true;
}

int main()
{
    std::vector<std::pair<int64_t, int64_t>> data;
    std::string line;

    while (std::getline(std::cin, line))
    {
        int64_t a, b;
        char comma;
        std::stringstream ss(line);
        ss >> a >> comma >> b;
        data.push_back({a, b});
    }

    int64_t part1 = 0;
    int64_t part2 = 0;

    for (size_t i = 0; i < data.size(); i++)
    {
        for (size_t j = i + 1; j < data.size(); j++)
        {
            // Compute area
            int64_t dx = std::abs(data[j].first - data[i].first) + 1;
            int64_t dy = std::abs(data[j].second - data[i].second) + 1;
            int64_t area = dx * dy;
            part1 = std::max(part1, area);
            // Check if points define valid rectangle
            if (area == 2) // Two points are adjacent
                std::cerr << "Validity of the test might be compromised!\n";
            if (is_valid(data, i, j))
                part2 = std::max(part2, area);
        }
    }

    // PART 1
    std::cout << "Part 1: " << part1 << "\n";

    // PART 2
    std::cout << "Part 2: " << part2 << "\n";

    return 0;
}
