# Compiler and flags
CXX := g++
CXXFLAGS := -std=c++17 -Wall -Wextra -O2

# Directories
DAYS_DIR := days
BIN_DIR := bin
INPUT_DIR := input

# Find all day source files
SOURCES := $(wildcard $(DAYS_DIR)/day*.cpp)
TARGETS := $(patsubst $(DAYS_DIR)/day%.cpp,$(BIN_DIR)/day%,$(SOURCES))

# Default target: build all days
all: $(TARGETS)

# Rule to build each day's executable
$(BIN_DIR)/day%: $(DAYS_DIR)/day%.cpp | $(BIN_DIR)
	$(CXX) $(CXXFLAGS) $< -o $@

# Create bin directory if it doesn't exist
$(BIN_DIR):
	mkdir -p $(BIN_DIR)

# Run a specific day (usage: make run DAY=01 [TEST=1])
run: $(BIN_DIR)/day$(DAY)
ifndef DAY
	$(error DAY is not defined. Usage: make run DAY=01 [TEST=1])
endif
	@FILE="$(INPUT_DIR)/day$(DAY).txt"; \
	if [ -n "$(TEST)" ]; then FILE="$(INPUT_DIR)/day$(DAY)_test.txt"; fi; \
	if [ ! -f "$$FILE" ]; then echo "Input file not found: $$FILE" && exit 1; fi; \
	$< < "$$FILE"

# Clean build artifacts
clean:
	rm -rf $(BIN_DIR)

# Rebuild everything
rebuild: clean all

.PHONY: all run clean rebuild
