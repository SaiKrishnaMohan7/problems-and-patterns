# Claude.md - Problems and Patterns Infrastructure

## Repository Purpose

This repository serves as the source of truth for algorithm and data structure problems implemented across TypeScript, Go, and Rust. It's designed to support a spaced repetition learning system via an iPhone app that fetches problems directly from GitHub.

## Folder Structure

```
problems-and-patterns/
├── README.md
├── Makefile
├── Claude.md (this file)
├── Specification.md
├── real-world-applications.json
├── package.json
├── tsconfig.json
├── jest.config.js
├── go.mod
├── go.sum
├── Cargo.toml
├── Cargo.lock
├── problems/
│   ├── arrays-hashing/
│   │   ├── contains-duplicate/
│   │   │   ├── problem.md
│   │   │   ├── solution.ts
│   │   │   ├── solution.test.ts
│   │   │   ├── solution.go
│   │   │   ├── solution_test.go
│   │   │   ├── solution.rs
│   │   │   ├── Cargo.toml
│   │   │   └── metadata.json
│   │   ├── valid-anagram/
│   │   │   ├── problem.md
│   │   │   ├── solution.ts
│   │   │   ├── solution.test.ts
│   │   │   ├── solution.go
│   │   │   ├── solution_test.go
│   │   │   ├── solution.rs
│   │   │   ├── Cargo.toml
│   │   │   └── metadata.json
│   │   └── ...
│   ├── two-pointers/
│   │   └── ...
│   ├── sliding-window/
│   │   └── ...
│   ├── stack/
│   │   └── ...
│   ├── binary-search/
│   │   └── ...
│   ├── linked-list/
│   │   └── ...
│   ├── trees/
│   │   └── ...
│   ├── tries/
│   │   └── ...
│   ├── heap-priority-queue/
│   │   └── ...
│   ├── backtracking/
│   │   └── ...
│   ├── graphs/
│   │   └── ...
│   ├── advanced-graphs/
│   │   └── ...
│   ├── 1d-dp/
│   │   └── ...
│   ├── 2d-dp/
│   │   └── ...
│   ├── greedy/
│   │   └── ...
│   ├── intervals/
│   │   └── ...
│   ├── math-geometry/
│   │   └── ...
│   └── bit-manipulation/
│       └── ...
└── templates/
    ├── problem.template.md
    ├── solution.template.ts
    ├── solution.test.template.ts
    ├── solution.template.go
    ├── solution_test.template.go
    ├── solution.template.rs
    ├── cargo.template.toml
    ├── metadata.template.json
    └── quiz-generation.template.json
```

## File Specifications

### metadata.json

Each problem includes a `metadata.json` file with the following structure:

```json
{
  "title": "Contains Duplicate",
  "slug": "contains-duplicate",
  "difficulty": "Easy",
  "pattern": "Arrays & Hashing",
  "neetcode_url": "https://neetcode.io/problems/duplicate-integer",
  "leetcode_url": "https://leetcode.com/problems/contains-duplicate/",
  "tags": ["array", "hash-table"],
  "time_complexity": "O(n)",
  "space_complexity": "O(n)",
  "description": "Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.",
  "hints": [
    "Can you use a hash set to track seen elements?",
    "What's the time complexity of hash set lookups?"
  ],
  "real_world_applications": [
    "Detecting duplicate user registrations (email/username validation)",
    "Finding duplicate files in a file system by hash",
    "Identifying duplicate transactions in payment processing",
    "Cache key collision detection"
  ]
}
```

### problem.md

The problem statement in markdown format. Should include:
- Problem description
- Examples with input/output
- Constraints
- Follow-up questions (if any)

### solution.[ts|go|rs]

Implementation files in each language. Should include:
- Function signature matching the problem requirements
- Comments explaining the approach
- Test cases if applicable

### Test Files

Each solution should have corresponding test files:

**TypeScript (`solution.test.ts`):**
- Uses Jest testing framework
- Imports the solution function
- Includes test cases with expected inputs/outputs
- Tests edge cases

**Go (`solution_test.go`):**
- Uses Go's standard testing package
- Follows Go testing conventions
- Table-driven tests preferred
- Tests edge cases

**Rust (`solution.rs` or separate test file):**
- Uses Rust's built-in testing framework
- Tests can be in the same file (Rust convention) or separate `tests/` directory
- Uses `#[cfg(test)]` module
- Tests edge cases

## Testing Infrastructure

### Language-Specific Setup

**TypeScript:**
- `package.json`: Node.js project with TypeScript and Jest dependencies
- `tsconfig.json`: TypeScript compiler configuration
- `jest.config.js`: Jest testing configuration
- All TypeScript solutions testable via `npm test` or `make test-ts`

**Go:**
- `go.mod`: Go module at repository root
- All problems under single module with proper package structure
- Each problem's Go code in its own package
- All Go solutions testable via `go test ./...` or `make test-go`

**Rust:**
- `Cargo.toml`: Workspace configuration at repository root
- Each problem as a library crate in the workspace
- Workspace members include all problem directories with Rust solutions
- All Rust solutions testable via `cargo test` or `make test-rs`

### Testing Requirements

**All solutions must:**
1. Be syntactically valid and compile/parse without errors
2. Include at least 3 test cases:
   - Basic case from problem examples
   - Edge case (empty input, single element, etc.)
   - Complex case
3. Pass all test cases before committing
4. Follow language-specific conventions and idioms

**Test file naming:**
- TypeScript: `solution.test.ts`
- Go: `solution_test.go`
- Rust: Tests in `solution.rs` with `#[cfg(test)]` or separate `tests/` directory

### quiz-generation.template.json

Defines rules for dynamically generating quiz questions from problem data:

```json
{
  "question_types": {
    "code_selection": {
      "enabled": true,
      "prompt_template": "Which solution correctly implements {{TITLE}}?",
      "generation_strategy": "show_correct_solution_plus_3_variations",
      "variations": [
        "wrong_data_structure",
        "off_by_one_error",
        "incorrect_edge_case_handling"
      ]
    },
    "complexity_analysis": {
      "enabled": true,
      "prompt_template": "What is the time complexity of {{TITLE}}?",
      "generation_strategy": "use_metadata_complexity",
      "wrong_options_count": 3
    },
    "pattern_identification": {
      "enabled": true,
      "prompt_template": "Which pattern does {{TITLE}} use?",
      "generation_strategy": "use_metadata_pattern_plus_similar_patterns"
    },
    "language_translation": {
      "enabled": true,
      "prompt_template": "Given this {{SOURCE_LANG}} solution for {{TITLE}}, which {{TARGET_LANG}} code is equivalent?",
      "generation_strategy": "show_correct_translation_plus_3_variations",
      "language_pairs": [
        ["go", "rust"],
        ["typescript", "go"],
        ["rust", "typescript"]
      ]
    },
    "bug_detection": {
      "enabled": true,
      "prompt_template": "Which line contains a bug in this solution for {{TITLE}}?",
      "generation_strategy": "inject_single_bug_into_correct_solution",
      "bug_types": [
        "off_by_one",
        "wrong_operator",
        "missing_edge_case",
        "incorrect_return"
      ]
    },
    "real_world_application": {
      "enabled": true,
      "prompt_template": "Which real-world scenario would benefit from the pattern used in {{TITLE}}?",
      "generation_strategy": "use_metadata_applications_plus_unrelated_options"
    }
  },
  "difficulty_scaling": {
    "easy": {
      "question_types": ["complexity_analysis", "pattern_identification", "real_world_application"],
      "time_limit_seconds": 30
    },
    "medium": {
      "question_types": ["code_selection", "language_translation", "real_world_application"],
      "time_limit_seconds": 45
    },
    "hard": {
      "question_types": ["code_selection", "bug_detection", "language_translation"],
      "time_limit_seconds": 60
    }
  }
}
```

The iPhone app uses this template to generate quiz questions on-the-fly without requiring manually written questions for each problem.

## Real-World Applications Database

### real-world-applications.json

A comprehensive mapping of patterns and problems to their real-world use cases. This JSON file serves as a reference for:

1. **Populating metadata.json** - When creating a new problem, reference this file to add appropriate `real_world_applications` entries
2. **Quiz generation** - The app uses this data to generate "Real-World Application" quiz questions
3. **Learning context** - Helps connect abstract algorithms to concrete systems engineering problems

**Structure:**
```json
{
  "patterns": {
    "pattern-name": {
      "pattern_applications": [
        "High-level uses of this pattern category"
      ],
      "problems": {
        "problem-slug": [
          "Specific real-world application 1",
          "Specific real-world application 2",
          "..."
        ]
      }
    }
  }
}
```

**Usage:**
When adding a new problem, lookup the pattern and problem slug in `real-world-applications.json` and copy 3-5 relevant applications to the problem's `metadata.json` file.

**iPhone App Integration:**
The app can optionally fetch this file to:
- Display additional context for problems
- Generate quiz questions about real-world applications
- Show suggestions when users are taking notes

This file is not required for core app functionality but enhances the learning experience by connecting theory to practice.

## Makefile Targets

### `make new`

Generates a new problem folder with all necessary files.

**Usage:**
```bash
make new PATTERN=arrays-hashing SLUG=contains-duplicate TITLE="Contains Duplicate"
```

**What it does:**
1. Creates the problem directory: `problems/{PATTERN}/{SLUG}/`
2. Copies template files and replaces placeholders
3. Creates `problem.md`, `solution.ts`, `solution.test.ts`, `solution.go`, `solution_test.go`, `solution.rs`, `Cargo.toml`, and `metadata.json`
4. Opens the files for editing

**Required variables:**
- `PATTERN`: The pattern category (e.g., arrays-hashing, two-pointers)
- `SLUG`: URL-friendly problem name (e.g., contains-duplicate)
- `TITLE`: Human-readable title (e.g., "Contains Duplicate")

**Optional variables:**
- `DIFFICULTY`: Easy, Medium, or Hard (default: Medium)
- `NEETCODE_URL`: Link to NeetCode problem
- `LEETCODE_URL`: Link to LeetCode problem

### `make list`

Lists all problems organized by pattern.

**Usage:**
```bash
make list
```

### `make validate`

Validates that all problems have the required files and correct structure.

**Usage:**
```bash
make validate
```

**Checks:**
- Each problem directory has all required files
- metadata.json is valid JSON
- All solution files exist and are not empty
- Pattern directories match expected categories

### `make test-ts`

Runs TypeScript solutions tests using Jest.

**Usage:**
```bash
make test-ts
```

**What it does:**
- Runs `npm test` or `jest`
- Executes all `*.test.ts` files
- Shows pass/fail status for each test

**Optional: Test specific problem:**
```bash
make test-ts PROBLEM=contains-duplicate
```

### `make test-go`

Runs Go solutions tests.

**Usage:**
```bash
make test-go
```

**What it does:**
- Runs `go test ./problems/...`
- Executes all Go test files
- Shows pass/fail status

**Optional: Test specific problem:**
```bash
make test-go PROBLEM=contains-duplicate
```

### `make test-rs`

Runs Rust solutions tests.

**Usage:**
```bash
make test-rs
```

**What it does:**
- Runs `cargo test`
- Executes all Rust tests in workspace
- Shows pass/fail status

**Optional: Test specific problem:**
```bash
make test-rs PROBLEM=contains-duplicate
```

### `make test`

Runs all tests across all languages.

**Usage:**
```bash
make test
```

**What it does:**
- Runs `make test-ts`
- Runs `make test-go`
- Runs `make test-rs`
- Reports overall pass/fail status

## Templates

Template files use placeholder variables that get replaced during generation:

- `{{TITLE}}`: Problem title
- `{{SLUG}}`: URL-friendly slug
- `{{PATTERN}}`: Pattern category
- `{{DIFFICULTY}}`: Problem difficulty
- `{{DESCRIPTION}}`: Problem description

## GitHub Integration

The iPhone app syncs the entire repository by downloading it as a zip archive:

**Download repository archive:**
```
GET https://github.com/{owner}/problems-and-patterns/archive/refs/heads/main.zip
```

This returns the entire repository as a compressed zip file.

**Sync Process:**
1. Download the zip archive from GitHub
2. Unzip to local storage
3. Parse the directory structure to find all patterns and problems
4. Read `metadata.json`, `problem.md`, and solution files directly from local filesystem
5. Load `real-world-applications.json` for additional context
6. Import into Core Data or cache
7. Delete the unzipped files (keep only parsed data in app storage)

**Benefits:**
- Single HTTP request instead of 750+ API calls
- No rate limiting concerns
- Faster sync (bulk download vs individual file requests)
- Simpler code (standard file I/O instead of API parsing)
- No base64 decoding needed
- Works great with manual sync model

**File paths after unzip:**
```
problems-and-patterns-main/
├── problems/
│   ├── arrays-hashing/
│   │   ├── contains-duplicate/
│   │   │   ├── problem.md
│   │   │   ├── solution.ts
│   │   │   ├── solution.go
│   │   │   ├── solution.rs
│   │   │   └── metadata.json
│   │   └── ...
│   └── ...
├── real-world-applications.json
└── ...
```

The app traverses this local structure to import all problems.

## Workflow

1. **Solve a problem** in all three languages (TS, Go, Rust)
2. **Run `make new`** to generate the problem structure
3. **Fill in the generated files** with your solutions and problem details
4. **Write tests** for each language implementation
5. **Run `make test`** to validate all solutions pass
6. **Commit and push** to GitHub
7. **Sync in the iPhone app** to fetch new problems
8. **Review daily** using the app's various study modes

## Development Notes

- Keep solution files focused and clean
- Include comments explaining the approach, especially for complex algorithms
- Use consistent naming conventions across languages
- **All solutions must have tests and pass before committing**
- **Run `make test` before every commit to ensure nothing breaks**
- Add meaningful test cases covering edge cases
- Update metadata.json with accurate complexity analysis
- Follow language-specific idioms and best practices

## Future Enhancements

- **CI/CD pipeline to validate solutions on commit** (GitHub Actions running `make test`)
- Code coverage reports for test completeness
- Automated linting and formatting checks
- Performance benchmarking for solutions
- Generate difficulty ratings based on time complexity
- Track problem-solving statistics
- Export functionality for other formats (Anki, etc.)
