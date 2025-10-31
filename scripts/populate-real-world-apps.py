#!/usr/bin/env python3
"""
Populate metadata.json files with real-world applications from real-world-applications.json
"""

import json
import sys
import os
from pathlib import Path

def load_real_world_apps(repo_root):
    """Load the real-world-applications.json file"""
    apps_file = repo_root / "real-world-applications.json"
    if not apps_file.exists():
        print(f"Error: {apps_file} not found", file=sys.stderr)
        sys.exit(1)

    with open(apps_file, 'r') as f:
        return json.load(f)

def get_real_world_apps(apps_data, pattern, slug):
    """Get real-world applications for a specific pattern and problem slug"""
    if "patterns" not in apps_data:
        return []

    patterns = apps_data["patterns"]

    if pattern not in patterns:
        print(f"Warning: Pattern '{pattern}' not found in real-world-applications.json", file=sys.stderr)
        return []

    pattern_data = patterns[pattern]

    if "problems" not in pattern_data:
        return []

    problems = pattern_data["problems"]

    if slug not in problems:
        print(f"Warning: Problem '{slug}' not found under pattern '{pattern}' in real-world-applications.json", file=sys.stderr)
        return []

    return problems[slug]

def update_metadata(metadata_path, real_world_apps):
    """Update metadata.json with real-world applications"""
    with open(metadata_path, 'r') as f:
        metadata = json.load(f)

    metadata["real_world_applications"] = real_world_apps

    with open(metadata_path, 'w') as f:
        json.dump(metadata, f, indent=2)
        f.write('\n')  # Add trailing newline

    return True

def populate_single_problem(repo_root, pattern, slug, apps_data):
    """Populate a single problem's metadata.json"""
    metadata_path = repo_root / "problems" / pattern / slug / "metadata.json"

    if not metadata_path.exists():
        print(f"Error: {metadata_path} not found", file=sys.stderr)
        return False

    real_world_apps = get_real_world_apps(apps_data, pattern, slug)

    if not real_world_apps:
        print(f"No real-world applications found for {pattern}/{slug}", file=sys.stderr)
        return False

    update_metadata(metadata_path, real_world_apps)
    print(f"✓ Updated {pattern}/{slug} with {len(real_world_apps)} real-world applications")
    return True

def populate_all_problems(repo_root, apps_data):
    """Populate all metadata.json files in the repository"""
    problems_dir = repo_root / "problems"

    if not problems_dir.exists():
        print("Error: problems/ directory not found", file=sys.stderr)
        sys.exit(1)

    updated_count = 0
    skipped_count = 0

    for pattern_dir in sorted(problems_dir.iterdir()):
        if not pattern_dir.is_dir():
            continue

        pattern = pattern_dir.name

        for problem_dir in sorted(pattern_dir.iterdir()):
            if not problem_dir.is_dir():
                continue

            slug = problem_dir.name
            metadata_path = problem_dir / "metadata.json"

            if not metadata_path.exists():
                print(f"Skipping {pattern}/{slug}: no metadata.json", file=sys.stderr)
                skipped_count += 1
                continue

            real_world_apps = get_real_world_apps(apps_data, pattern, slug)

            if not real_world_apps:
                print(f"Skipping {pattern}/{slug}: no real-world applications found")
                skipped_count += 1
                continue

            update_metadata(metadata_path, real_world_apps)
            print(f"✓ Updated {pattern}/{slug} with {len(real_world_apps)} real-world applications")
            updated_count += 1

    print(f"\nSummary: {updated_count} updated, {skipped_count} skipped")
    return updated_count > 0

def main():
    # Determine repository root (parent of scripts directory)
    repo_root = Path(__file__).parent.parent

    # Load real-world applications data
    apps_data = load_real_world_apps(repo_root)

    # Check if specific pattern and slug were provided
    if len(sys.argv) == 3:
        # Update single problem
        pattern = sys.argv[1]
        slug = sys.argv[2]
        success = populate_single_problem(repo_root, pattern, slug, apps_data)
        sys.exit(0 if success else 1)
    elif len(sys.argv) == 1:
        # Update all problems
        print("Populating real-world applications for all problems...")
        success = populate_all_problems(repo_root, apps_data)
        sys.exit(0 if success else 1)
    else:
        print("Usage:")
        print("  python3 populate-real-world-apps.py                    # Update all problems")
        print("  python3 populate-real-world-apps.py <pattern> <slug>   # Update specific problem")
        print()
        print("Examples:")
        print("  python3 populate-real-world-apps.py")
        print("  python3 populate-real-world-apps.py arrays-hashing contains-duplicate")
        sys.exit(1)

if __name__ == "__main__":
    main()
