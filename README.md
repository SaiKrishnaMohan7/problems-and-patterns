# Problems and Patterns

A comprehensive algorithm and data structure problem collection implemented in **TypeScript**, **Go**, and **Rust**. Designed to support a spaced repetition learning system via an iPhone app that fetches problems directly from GitHub.

## Overview

This repository serves as the source of truth for algorithm problems organized by patterns (Arrays & Hashing, Two Pointers, Sliding Window, etc.). Each problem includes:

- Problem statement and examples
- Solutions in TypeScript, Go, and Rust
- Comprehensive test suites
- Metadata (difficulty, complexity, real-world applications)

## Quick Start

### Prerequisites

- Node.js 20+ (for TypeScript)
- Go 1.25+ (for Go)
- Rust (latest stable) (for Rust)

### Installation

```bash
# Install dependencies
npm install

# Verify setup
make test
```

## Creating a New Problem

Use the `make new` command to generate a problem structure:

```bash
make new PATTERN=arrays-hashing SLUG=contains-duplicate TITLE="Contains Duplicate"
```

Optional parameters:
- `DIFFICULTY=Easy|Medium|Hard` (default: Medium)
- `NEETCODE_URL=https://...`
- `LEETCODE_URL=https://...`

This creates:
```
problems/arrays-hashing/contains-duplicate/
├── problem.md              # Problem statement
├── solution.ts             # TypeScript solution
├── solution.test.ts        # TypeScript tests (Vitest)
├── solution.go             # Go solution
├── solution_test.go        # Go tests
├── solution.rs             # Rust solution
├── Cargo.toml              # Rust package config
└── metadata.json           # Problem metadata
```

## Testing

Run all tests across all languages:

```bash
make test
```

Test specific languages:

```bash
make test-ts              # TypeScript tests (Vitest)
make test-go              # Go tests
make test-rs              # Rust tests
```

Test a specific problem:

```bash
make test-ts PROBLEM=contains-duplicate
make test-go PROBLEM=contains-duplicate
make test-rs PROBLEM=contains-duplicate
```

## Repository Structure

```
problems-and-patterns/
├── problems/                  # Problem solutions by pattern
│   ├── arrays-hashing/
│   ├── two-pointers/
│   ├── sliding-window/
│   ├── stack/
│   ├── binary-search/
│   ├── linked-list/
│   ├── trees/
│   ├── tries/
│   ├── heap-priority-queue/
│   ├── backtracking/
│   ├── graphs/
│   ├── advanced-graphs/
│   ├── 1d-dp/
│   ├── 2d-dp/
│   ├── greedy/
│   ├── intervals/
│   ├── math-geometry/
│   └── bit-manipulation/
├── templates/                 # Templates for new problems
├── real-world-applications.json  # Real-world use cases
├── Makefile                   # Build and test automation
├── package.json               # Node.js dependencies
├── tsconfig.json              # TypeScript configuration
├── vitest.config.ts           # Vitest test configuration
├── go.mod                     # Go module
├── Cargo.toml                 # Rust workspace
└── Claude.md                  # Detailed documentation
```

## Other Commands

List all problems:

```bash
make list
```

Validate repository structure:

```bash
make validate
```

Get help:

```bash
make help
```

## Workflow

1. **Create a problem**: `make new PATTERN=... SLUG=... TITLE="..."`
2. **Implement solutions** in all three languages
3. **Write tests** for each implementation
4. **Run tests**: `make test`
5. **Commit and push** to GitHub
6. **Sync in iPhone app** to fetch new problems

## Documentation

- **Claude.md**: Complete specification and implementation guide
- **Specification.md**: iPhone app specification and features
- **real-world-applications.json**: Mapping of problems to real-world systems use cases

## iPhone App Integration

The iOS app syncs by downloading the entire repository as a zip archive:

```
https://github.com/{owner}/problems-and-patterns/archive/refs/heads/main.zip
```

This provides:
- Single HTTP request (no rate limits)
- Faster sync (bulk download vs individual file requests)
- Simpler code (standard file I/O)
- Offline support

## Contributing

All solutions must:
1. Be syntactically valid and compile/parse without errors
2. Include at least 3 test cases (basic, edge, complex)
3. Pass all tests before committing
4. Follow language-specific conventions and idioms

Run `make test` before every commit!

## License

MIT
