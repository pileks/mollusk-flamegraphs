# Solana BPF Test Profiling Script

## Overview
The `scripts/programs/profile.sh` script runs Solana BPF tests with SBPF profiling enabled and saves flamegraphs with meaningful, organized names.

## Key Features

### 1. Serial Test Execution
- Uses `--test-threads=1` to run tests in series (not parallel)
- This ensures proper flamegraph tracking and avoids mixing outputs from concurrent tests

### 2. Instruction Name Extraction
- Extracts instruction names from the first "Program log:" line that appears after each "invoke [1]"
- The pattern is:
  ```
  Program HeLot... invoke [1]
  Program log: HelloWorld    <-- This is the instruction name
  Program log: Hello, World!
  ```
- Instruction names are converted to lowercase in the filename

### 3. Multiple Flamegraphs Per Test
- Handles tests that run multiple instructions sequentially
- Each flamegraph is indexed (0, 1, 2...) based on the order within the test
- Uses an associative array to track counters per test

### 4. Flexible Naming Convention

#### Single Test Mode
When running a specific test: `./scripts/programs/profile.sh test_hello_world_multiple`
- Format: `{test_name}_{index}_{instruction}.svg`
- Example: `test_hello_world_multiple_0_helloworld.svg`

#### All Tests Mode
When running all tests: `./scripts/programs/profile.sh`
- Format: `{full_test_path}_{index}_{instruction}.svg`
- Example: `processor_hello_world_tests_test_hello_world_multiple_0_helloworld.svg`

### 5. Test Name Detection
- Looks backward from each flamechart to find the test line
- Matches lines that start with `test ` and contain `...`
- Handles test lines that have debug output after `...` (common pattern)
- Example patterns it matches:
  ```
  test processor::hello_world::tests::test_hello_world ... [DEBUG output]
  test test_id ... ok
  ```

## Usage

```bash
# Run all tests with profiling
./scripts/programs/profile.sh

# Run specific test with profiling
./scripts/programs/profile.sh test_hello_world_multiple

# Custom output directory
./scripts/programs/profile.sh -o benchmarks test_hello_world

# Different program name
./scripts/programs/profile.sh -p my_program test_function

# Help
./scripts/programs/profile.sh -h
```

## Output Structure

Flamegraphs are saved to the `perf/` directory (or custom directory via `-o`) with the following structure:

```
perf/
├── test_hello_world_0_helloworld.svg              # Single test mode
├── processor_hello_world_tests_test_hello_world_0_helloworld.svg  # All tests mode
├── processor_hello_world_tests_test_hello_world_multiple_0_helloworld.svg
├── processor_hello_world_tests_test_hello_world_multiple_1_helloworld.svg
└── processor_hello_world_tests_test_hello_world_multiple_2_helloworld.svg
```

## Technical Implementation Details

### Key Parsing Logic

1. **Finding Instruction Names**:
   - Searches backward from each `[SBPF Profiler]` line
   - Finds the most recent `invoke [1]` line
   - Then searches forward from that invoke to find the first `Program log:` line
   - Extracts the first word after `Program log:` as the instruction name

2. **Finding Test Names**:
   - Searches backward from each flamechart to find test start lines
   - Uses regex: `^test[[:space:]]+(.*)[[:space:]]+\\.\\.\\. `
   - Extracts everything between `test ` and ` ...` as the test name
   - Stops searching if it hits another `[SBPF Profiler]` line (boundary)

3. **Indexing Flamecharts**:
   - Maintains an associative array `test_flamechart_counters`
   - Tracks the count of flamecharts per test
   - Resets when processing a new test

### Important Notes

- The script requires the program to be built (`cargo build-sbf`) and will attempt to build it if not found
- Original flamechart files (with UUID names) are deleted after copying to save space
- The script uses absolute paths for `SBPF_PROFILE` environment variable
- Color-coded output for better readability in terminal

## Common Issues and Solutions

1. **Unknown flamechart names**: Usually means the test completed too quickly or doesn't have program invocations
2. **Missing flamecharts**: Ensure `SBPF_PROFILE` is set to the correct absolute path of the `.so` file
3. **Mixed up test names**: Make sure to use `--test-threads=1` for serial execution

## Example Test Structure

For a test that generates multiple flamecharts:

```rust
#[test]
fn test_hello_world_multiple() {
    let mollusk = Mollusk::new(&crate::ID, "hello_world_program");

    let instruction = /* ... */;

    // Each of these generates a flamechart
    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}
```

This would generate:
- `test_hello_world_multiple_0_helloworld.svg`
- `test_hello_world_multiple_1_helloworld.svg`
- `test_hello_world_multiple_2_helloworld.svg`