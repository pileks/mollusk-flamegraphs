# Solana BPF Profiling with Mollusk and SBPF Profiler

Example project demonstrating performance profiling of Solana programs using [Mollusk](https://github.com/buffalojoec/mollusk) for testing and [sbpf-profiler](https://github.com/serbangv/sbpf-profiler) for flamegraph generation.

## Quick Start

```bash
# Build the program
bun run programs:build

# Run tests
bun run programs:test

# Generate performance flamegraphs
bun run programs:profile
```

## What It Does

- **Mollusk** provides a lightweight SVM environment for testing Solana programs
- **SBPF Profiler** captures performance data during program execution
- The profile script automatically generates flamegraphs for each instruction invocation, saving them with descriptive names like `test_hello_world_0_helloworld.svg`

## Viewing Results

Flamegraphs are saved to the `perf/` directory. Open the SVG files in a browser to explore the interactive performance profiles.

## Profile Script Options

```bash
bun run programs:profile -- [test_name]           # Profile specific test
bun run programs:profile -- -o benchmarks         # Custom output directory
bun run programs:profile -- -h                    # Show all options
```

## How It Works

1. The program is built with the patched `solana-sbpf` (see `Cargo.toml`)
2. Tests run with `SBPF_PROFILE` environment variable enabled
3. Each instruction execution generates a flamegraph
4. The script organizes outputs with meaningful names based on test and instruction