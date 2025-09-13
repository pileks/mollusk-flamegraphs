#!/bin/bash

# Script to run Solana BPF tests with profiling and save flamegraphs with meaningful names

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
PERF_DIR="perf"
PROGRAM_NAME="hello_world_program"

# Function to display usage
usage() {
    echo "Usage: $0 [OPTIONS] [test_name]"
    echo ""
    echo "Run Solana BPF tests with profiling and save the flamegraph"
    echo ""
    echo "Arguments:"
    echo "  test_name         Name of the test to run (optional, runs all if omitted)"
    echo ""
    echo "Options:"
    echo "  -o, --output DIR  Output directory for flamegraphs (default: perf)"
    echo "  -p, --program NAME Program name (default: hello_world_program)"
    echo "  -h, --help        Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all tests"
    echo "  $0 test_hello_world   # Run specific test"
    echo "  $0 -o benchmarks test_hello_world"
    echo "  $0 --program my_program test_function"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            PERF_DIR="$2"
            shift 2
            ;;
        -p|--program)
            PROGRAM_NAME="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo -e "${RED}Error: Unknown option $1${NC}"
            usage
            exit 1
            ;;
        *)
            TEST_NAME="$1"
            shift
            ;;
    esac
done

# Build the program path (absolute)
PROGRAM_PATH="$(pwd)/target/sbpf-solana-solana/release/${PROGRAM_NAME}.so"

# Check if program exists
if [ ! -f "$PROGRAM_PATH" ]; then
    echo -e "${YELLOW}Warning: Program not found at $PROGRAM_PATH${NC}"
    echo "Running cargo build-sbf first..."
    cargo build-sbf
    if [ ! -f "$PROGRAM_PATH" ]; then
        echo -e "${RED}Error: Failed to build program${NC}"
        exit 1
    fi
fi

# Determine test command
if [ -z "$TEST_NAME" ]; then
    echo -e "${GREEN}Running all tests${NC}"
    TEST_CMD="cargo test-sbf"
    OUTPUT_SUFFIX="all_tests"
else
    echo -e "${GREEN}Running test:${NC} $TEST_NAME"
    TEST_CMD="cargo test-sbf $TEST_NAME"
    OUTPUT_SUFFIX="$TEST_NAME"
fi

echo -e "${GREEN}Program:${NC} $PROGRAM_PATH"
echo -e "${GREEN}Output directory:${NC} $PERF_DIR"
echo ""

# Run the test with profiling and capture output
# Use --test-threads=1 to run tests in series for proper flamegraph tracking
output=$(SBPF_PROFILE="$PROGRAM_PATH" $TEST_CMD -- --nocapture --test-threads=1 2>&1)
test_exit_code=$?

# Display the test output
echo "$output"

# Check if test passed
if [ $test_exit_code -ne 0 ]; then
    echo -e "${RED}Test failed with exit code $test_exit_code${NC}"
    exit $test_exit_code
fi

# Create output directory if it doesn't exist
mkdir -p "$PERF_DIR"

# Extract all flamechart paths and their corresponding test names
flamechart_count=0
current_test_name=""
declare -A test_flamechart_counters

# Convert output to array of lines for easier processing
mapfile -t lines <<< "$output"

for i in "${!lines[@]}"; do
    line="${lines[$i]}"

    if [[ "$line" == *"[SBPF Profiler]"* ]] && [[ "$line" == *"Flamechart written to:"* ]]; then
        flamechart_path=$(echo "$line" | grep -oP '(?<=Flamechart written to: ")[^"]+')

        # Look BACKWARDS to find the instruction name
        # Find the most recent "invoke [1]" line, then look for the next "Program log:" line
        instruction_name=""
        for j in $(seq $((i - 1)) -1 0); do
            prev_line="${lines[$j]}"

            # Check if we've found an invoke line
            if [[ "$prev_line" == *"invoke [1]"* ]]; then
                # Now look forward from the invoke line to find the first Program log
                for k in $(seq $((j + 1)) $i); do
                    check_line="${lines[$k]}"
                    if [[ "$check_line" == *"Program log:"* ]]; then
                        # Extract the instruction name (first word after "Program log:")
                        instruction_name=$(echo "$check_line" | sed -n 's/.*Program log: \([A-Za-z0-9_]*\).*/\1/p')
                        break 2  # Break out of both loops
                    fi
                    # Stop if we hit the profiler line we're processing
                    if [ $k -eq $i ]; then
                        break
                    fi
                done
            fi
        done

        # Look BACKWARD for the test name (test lines appear before flamecharts)
        test_name=""
        for j in $(seq $((i - 1)) -1 0); do
            prev_line="${lines[$j]}"
            # Match test line that starts with "test " and has "..." (may have debug output after)
            if [[ "$prev_line" =~ ^test[[:space:]]+(.*)[[:space:]]+\.\.\. ]]; then
                # Extract the test name (everything between "test " and " ...")
                test_name="${BASH_REMATCH[1]}"
                break
            fi
            # Stop if we hit another profiler output (we've gone too far back)
            if [[ "$prev_line" == *"[SBPF Profiler]"* ]]; then
                break
            fi
        done

        if [ -n "$flamechart_path" ]; then
            # Update current test name if we found one
            if [ -n "$test_name" ]; then
                current_test_name="$test_name"
            fi

            # Track flamechart index per test
            if [ -n "$current_test_name" ]; then
                if [ -z "${test_flamechart_counters[$current_test_name]}" ]; then
                    test_flamechart_counters[$current_test_name]=0
                else
                    test_flamechart_counters[$current_test_name]=$((test_flamechart_counters[$current_test_name] + 1))
                fi
                current_index=${test_flamechart_counters[$current_test_name]}
            else
                current_index=$flamechart_count
            fi

            # Clean up test name (replace :: with _ and remove spaces)
            clean_test_name=$(echo "$current_test_name" | sed 's/::/_/g' | sed 's/[[:space:]]//g')

            # Convert instruction name to lowercase
            instruction_lower=$(echo "${instruction_name:-unknown}" | tr '[:upper:]' '[:lower:]')

            # Generate output filename with index and instruction name
            if [ -n "$TEST_NAME" ]; then
                # Single test mode: use test_name_index_instruction format
                if [ -n "$instruction_name" ]; then
                    output_file="${PERF_DIR}/${TEST_NAME}_${current_index}_${instruction_lower}.svg"
                else
                    output_file="${PERF_DIR}/${TEST_NAME}_${current_index}_unknown.svg"
                fi
            else
                # All tests mode: include test name in filename
                if [ -n "$clean_test_name" ] && [ -n "$instruction_name" ]; then
                    output_file="${PERF_DIR}/${clean_test_name}_${current_index}_${instruction_lower}.svg"
                elif [ -n "$clean_test_name" ]; then
                    output_file="${PERF_DIR}/${clean_test_name}_${current_index}_unknown.svg"
                else
                    output_file="${PERF_DIR}/unknown_${flamechart_count}.svg"
                fi
            fi

            # Copy the flamechart to the new location
            cp "$flamechart_path" "$output_file"

            if [ $? -eq 0 ]; then
                echo -e "${GREEN}✓ Flamechart saved to: $output_file${NC}"
                flamechart_count=$((flamechart_count + 1))

                # Remove the original file to save space
                rm "$flamechart_path" 2>/dev/null
            else
                echo -e "${RED}Error: Failed to copy flamechart${NC}"
            fi
        fi
    fi
done

if [ $flamechart_count -eq 0 ]; then
    echo -e "${YELLOW}Warning: No flamecharts were generated${NC}"
    echo "Make sure the tests actually run and SBPF profiling is enabled"
else
    echo ""
    echo -e "${GREEN}Total flamecharts saved: $flamechart_count${NC}"
fi