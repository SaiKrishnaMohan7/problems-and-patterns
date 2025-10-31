# Specification.md - Problems and Patterns iPhone App

## Overview

An iOS application for mastering algorithm and data structure problems through spaced repetition and multi-modal learning. The app fetches problems from the `problems-and-patterns` GitHub repository and provides various study modes to build muscle memory for problem-solving patterns.

## Core Philosophy

**Goal**: Achieve "wake up at 3am and write it" level mastery of fundamental algorithms and data structures across TypeScript, Go, and Rust.

**Learning Method**: 
- Implement each problem in all three languages
- Daily visual review to maintain familiarity
- Active recall through blank implementation
- Pattern recognition through multiple choice quizzes
- Spaced repetition to combat forgetting

## Primary Features

### 1. Sync with GitHub

**Manual Sync Button**
- Downloads entire repository as zip archive
- Shows sync progress (downloading, unzipping, parsing)
- Caches all content locally for offline use
- Displays last sync timestamp
- Handles network errors gracefully

**Repository Configuration**
- Repository owner and name configured during setup
- Can be modified in settings
- Default branch: main
- Download URL format: `https://github.com/{owner}/{repo}/archive/refs/heads/main.zip`

**Sync Process**
1. Download repository zip from GitHub (`{owner}/{repo}/archive/refs/heads/main.zip`)
2. Unzip to temporary directory
3. Traverse `problems/` directory to discover all patterns and problems
4. For each problem directory, read:
   - `metadata.json`
   - `problem.md`
   - `solution.ts`
   - `solution.go`
   - `solution.rs`
5. Load `real-world-applications.json` from repo root
6. Parse and import all data into Core Data
7. Clean up temporary files (delete unzipped directory)
8. Update UI with new problems

**Benefits over API approach:**
- Single HTTP request (no rate limits)
- Faster sync (bulk download)
- Simpler implementation
- No base64 decoding
- Standard file I/O

### 2. Problem Browser

**Pattern-Based Navigation**
- List view organized by pattern categories (Arrays & Hashing, Two Pointers, etc.)
- Show problem count per pattern
- Visual indicator for completion status

**Problem List**
- Display all problems within a pattern
- Show: title, difficulty (color-coded), languages completed
- Filter by: difficulty, completion status, tags
- Sort by: difficulty, title, last reviewed date
- Search functionality

**Problem Detail View**
- Problem statement (rendered markdown)
- Difficulty badge
- Tags
- Time/space complexity
- **Real-world applications** (bulleted list showing where this pattern is used in practice)
- Links to NeetCode and LeetCode
- Last reviewed timestamp
- Personal notes section

### 3. Study Modes

#### Mode 1: Visual Review (Daily Scroll)
**Purpose**: Passive reinforcement through daily exposure

**Features**:
- Card-based interface showing one solution at a time
- Swipe to navigate between problems
- Toggle between languages (TS → Go → Rust)
- Syntax-highlighted code
- "Mark as reviewed" button
- Quick access to problem statement
- Scheduled daily via spaced repetition algorithm

**Use Case**: Morning coffee routine - spend 5-10 minutes scrolling through solutions

#### Mode 2: Blank Implementation (Active Recall)
**Purpose**: Test ability to implement from memory

**Features**:
- Shows problem statement only
- Timer starts automatically
- Blank code editor with language selector
- Minimal IDE features: syntax highlighting, basic autocomplete
- "Show Solution" button (marks as failed attempt)
- "Submit" button to compare against stored solution
- Diff view showing your solution vs. correct solution
- Statistics: time taken, success rate per problem

**Use Case**: Evening practice - implement 1-2 problems from scratch

#### Mode 3: Multiple Choice Quiz
**Purpose**: Pattern recognition and conceptual understanding

**Question Types** (dynamically generated):
1. **Code Selection**: "Which solution correctly solves [problem]?" - show correct solution + 3 variations with common mistakes
2. **Complexity Analysis**: "What's the time complexity?" - use metadata + generate 3 plausible wrong answers
3. **Pattern Identification**: "Which pattern does this problem use?" - show correct pattern + 3 similar patterns
4. **Language Translation**: "Given this Go solution, which Rust code is equivalent?" - show correct translation + 3 variations
5. **Bug Detection**: "Which line has the bug?" - inject a single bug into correct solution
6. **Real-World Application**: "Which scenario would benefit from this pattern?" - use metadata applications + unrelated options

**Dynamic Generation Strategy**:
- Questions are generated on-the-fly from existing solutions and metadata
- No manual question writing required
- Uses `quiz-generation.template.json` for generation rules
- Variations created by introducing common mistakes (off-by-one, wrong data structure, etc.)
- Difficulty-appropriate question types (easier problems get simpler question types)

**Features**:
- Random question generation from problem pool
- Immediate feedback (correct/incorrect)
- Explanation after each question (why answer is correct, what the bug was, etc.)
- Quiz sessions: 10/25/50 questions
- Statistics: accuracy per question type, per pattern
- Show real-world applications after correct answers to reinforce practical understanding

#### Mode 4: Language Comparison
**Purpose**: Understand language idioms and trade-offs

**Features**:
- Side-by-side view of solutions in all three languages
- Highlights differences in approach
- Shows language-specific optimizations
- Toggle individual language visibility
- Notes section for observations

**Use Case**: Weekend deep-dive into how languages differ

### 4. Spaced Repetition System

**Algorithm**: Modified Anki-style SM-2 algorithm

**Scheduling Logic**:
- New problems: review next day
- Successful review: interval increases (1d → 3d → 7d → 14d → 30d → 60d)
- Failed review: interval resets to 1 day
- Difficulty adjustment based on performance

**Review Queue**:
- "Due Today" counter on home screen
- Prioritizes problems due for review
- Balances between new problems and reviews
- Shows upcoming reviews for next 7 days

**Statistics**:
- Retention rate per pattern
- Problems mastered (60+ day interval)
- Review streak counter
- Time spent per study mode
- Heatmap of daily activity

### 5. Progress Tracking

**Dashboard**:
- Total problems completed
- Problems per pattern (progress bars)
- Mastery level per language
- Current streak
- Problems due today

**Charts**:
- Completion over time (line graph)
- Problems by difficulty (pie chart)
- Time spent per pattern (bar chart)
- Review success rate (trend line)

**Milestones**:
- First problem solved
- 10/50/100/150 problems completed
- 30/60/90 day streak
- All problems in a pattern mastered
- 100% completion in a language

### 6. Settings

**GitHub Integration**:
- Repository owner input (default: your username)
- Repository name input (default: problems-and-patterns)
- Branch name (default: main)
- Manual sync trigger
- Clear local cache option

**Study Preferences**:
- Default language for blank implementation
- Timer duration for practice mode
- Notifications for due reviews
- Daily review reminder time

**Display**:
- Theme: Light/Dark/System
- Font size for code
- Color scheme for syntax highlighting

**Data Management**:
- Export progress as JSON
- Import progress from backup
- Reset all progress (with confirmation)

## Technical Architecture

### Data Models

**Problem**:
```swift
struct Problem: Codable, Identifiable {
    let id: UUID
    let slug: String
    let title: String
    let pattern: String
    let difficulty: Difficulty
    let description: String
    let hints: [String]
    let tags: [String]
    let timeComplexity: String
    let spaceComplexity: String
    let neetcodeURL: String?
    let leetcodeURL: String?
    let realWorldApplications: [String]
    
    // Solutions
    let typescriptSolution: String
    let goSolution: String
    let rustSolution: String
    
    // User data
    var lastReviewed: Date?
    var nextReview: Date?
    var reviewCount: Int
    var successCount: Int
    var notes: String?
}

enum Difficulty: String, Codable {
    case easy = "Easy"
    case medium = "Medium"
    case hard = "Hard"
}
```

**ReviewSession**:
```swift
struct ReviewSession: Codable, Identifiable {
    let id: UUID
    let problemId: UUID
    let mode: StudyMode
    let timestamp: Date
    let duration: TimeInterval
    let success: Bool
    let attemptedCode: String?
}

enum StudyMode: String, Codable {
    case visualReview
    case blankImplementation
    case multipleChoice
    case languageComparison
}
```

**UserProgress**:
```swift
struct UserProgress: Codable {
    var totalProblems: Int
    var completedProblems: Int
    var currentStreak: Int
    var longestStreak: Int
    var reviewSessions: [ReviewSession]
    var lastSyncDate: Date?
}
```

### Storage

**Core Data** for structured data:
- Problems
- Review sessions
- Progress statistics
- Spaced repetition metadata

**File System** for code content:
- Solution files cached locally
- User's attempted solutions
- Backup exports
- Temporary zip downloads

### Networking

**GitHubService**:
```swift
class GitHubService {
    func downloadRepository(owner: String, repo: String) async throws -> URL
    func unzipRepository(zipURL: URL) async throws -> URL
    func parseRepository(repoURL: URL) async throws -> [Problem]
    func loadRealWorldApplications(repoURL: URL) async throws -> RealWorldApplicationsDB
}
```

**RepositoryParser**:
```swift
class RepositoryParser {
    func discoverPatterns(at url: URL) throws -> [String]
    func discoverProblems(pattern: String, at url: URL) throws -> [String]
    func parseProblem(pattern: String, slug: String, at url: URL) throws -> Problem
}
```

**RealWorldApplicationsDB**:
```swift
struct RealWorldApplicationsDB: Codable {
    let patterns: [String: PatternApplications]
}

struct PatternApplications: Codable {
    let patternApplications: [String]
    let problems: [String: [String]]
}
```

**Caching Strategy**:
- Check local cache first
- Fetch from GitHub only on manual sync
- Store raw API responses for debugging
- Implement cache invalidation on sync

### UI Components

**SwiftUI Views**:
- `HomeView`: Dashboard with stats and due reviews
- `PatternListView`: Browse problems by pattern
- `ProblemDetailView`: Full problem information
- `VisualReviewView`: Card-based solution review
- `BlankImplementationView`: Code editor for practice
- `MultipleChoiceView`: Quiz interface
- `LanguageComparisonView`: Side-by-side solutions
- `ProgressView`: Charts and statistics
- `SettingsView`: Configuration options

**Custom Components**:
- `CodeEditor`: Syntax-highlighted text editor
- `SyntaxHighlighter`: Language-specific highlighting
- `DiffViewer`: Side-by-side code comparison
- `ProgressChart`: Custom charts for statistics

### Key Technologies

- **SwiftUI**: UI framework
- **Core Data**: Local persistence
- **Combine**: Reactive programming
- **URLSession**: Networking (zip download)
- **ZipFoundation** or **Compression framework**: Unzipping archives
- **Swift Markdown**: Rendering problem statements
- **Highlight.js** or **CodeMirror**: Syntax highlighting
- **Charts framework**: Data visualization

## User Flows

### First Launch Flow
1. Welcome screen explaining the app
2. Prompt for repository owner and name (with defaults)
3. Initial sync from repository (download zip)
4. Quick tutorial of study modes
5. Show dashboard

### Daily Review Flow
1. Open app → see "X problems due today"
2. Tap "Start Review"
3. Visual Review mode: scroll through 5-10 solutions
4. Mark each as reviewed
5. Return to dashboard showing updated stats

### Practice Session Flow
1. Navigate to Blank Implementation mode
2. Choose a problem (random, specific, or due for review)
3. Select language
4. Timer starts, write solution
5. Submit and view diff
6. Save or discard attempt
7. Spaced repetition schedules next review

### Weekly Progress Check Flow
1. Navigate to Progress tab
2. View completion charts
3. Identify weak patterns
4. Filter problem list by weak pattern
5. Add to review queue

## Performance Requirements

- **Sync time**: < 10 seconds for 150 problems (single zip download + parse)
- **App launch**: < 2 seconds to home screen
- **Code rendering**: < 500ms for syntax highlighting
- **Search**: < 100ms for local search
- **Offline mode**: Full functionality except sync

## Accessibility

- VoiceOver support for all UI elements
- Dynamic Type support for text scaling
- High contrast mode support
- Keyboard shortcuts for code editor
- Haptic feedback for interactions

## Future Enhancements (Post-MVP)

- **Watch app**: Quick review queue on Apple Watch
- **Widget**: Today's due problems
- **Share progress**: Social features, leaderboards
- **Collaborative mode**: Compare solutions with friends
- **AI hints**: GPT-powered hints without giving away solution
- **Video solutions**: Link to video explanations
- **Custom problems**: Add your own problems
- **Export to Anki**: Generate Anki decks
- **Web version**: Access from browser
- **Problem of the Day**: Daily challenge notification

## Success Metrics

**Engagement**:
- Daily active users
- Average session length
- Review completion rate
- Streak maintenance

**Learning**:
- Problems mastered (60+ day interval)
- Blank implementation success rate
- Time to solve improvement over time
- Pattern coverage across languages

**Retention**:
- 7-day retention rate
- 30-day retention rate
- Churn rate after completing all problems

## Development Phases

### Phase 1: MVP (Weeks 1-2)
- GitHub sync functionality
- Basic problem browser
- Visual Review mode
- Local storage with Core Data
- Simple spaced repetition

### Phase 2: Practice Tools (Weeks 3-4)
- Blank Implementation mode with code editor
- Diff viewer
- Timer and statistics
- Enhanced spaced repetition algorithm

### Phase 3: Polish (Weeks 5-6)
- Multiple Choice quiz mode
- Language Comparison view
- Progress charts and dashboard
- Settings and customization
- Offline mode improvements

### Phase 4: Refinement (Ongoing)
- Performance optimization
- Bug fixes based on usage
- UI/UX improvements
- Additional features based on feedback

## Decisions Made

1. **Single repository format**: The app only supports the canonical `problems-and-patterns` repository structure. Repository owner and name configured during setup and modifiable in settings.

2. **GitHub API only**: No custom web API layer. App communicates directly with GitHub API for syncing.

3. **Dynamic quiz generation**: Questions are generated on-the-fly from solutions and metadata using the `quiz-generation.template.json` rules. No manual question writing required.

4. **No tracking of individual attempts**: The app focuses purely on memorization through spaced repetition. No detailed attempt history or code versioning.

5. **No social features**: This is a personal learning tool. No sharing, leaderboards, or community features.
