.PHONY: new list validate test test-ts test-go test-rs help

# Default values
DIFFICULTY ?= Medium

help:
	@echo "Available targets:"
	@echo "  make new PATTERN=<pattern> SLUG=<slug> TITLE=<title> [DIFFICULTY=<difficulty>] [NEETCODE_URL=<url>] [LEETCODE_URL=<url>]"
	@echo "  make list                     - List all problems by pattern"
	@echo "  make validate                 - Validate repository structure"
	@echo "  make test                     - Run all tests (TypeScript, Go, Rust)"
	@echo "  make test-ts [PROBLEM=<slug>] - Run TypeScript tests"
	@echo "  make test-go [PROBLEM=<slug>] - Run Go tests"
	@echo "  make test-rs [PROBLEM=<slug>] - Run Rust tests"

new:
	@if [ -z "$(PATTERN)" ] || [ -z "$(SLUG)" ] || [ -z "$(TITLE)" ]; then \
		echo "Error: PATTERN, SLUG, and TITLE are required"; \
		echo "Usage: make new PATTERN=arrays-hashing SLUG=contains-duplicate TITLE=\"Contains Duplicate\""; \
		exit 1; \
	fi
	@echo "Creating problem: $(TITLE)"
	@mkdir -p problems/$(PATTERN)/$(SLUG)
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g; s/{{PATTERN}}/$(PATTERN)/g; s/{{DIFFICULTY}}/$(DIFFICULTY)/g; s/{{DESCRIPTION}}/TODO: Add description/g' \
		templates/problem.template.md > problems/$(PATTERN)/$(SLUG)/problem.md
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g; s/{{PATTERN}}/$(PATTERN)/g; s/{{DIFFICULTY}}/$(DIFFICULTY)/g' \
		templates/solution.template.ts > problems/$(PATTERN)/$(SLUG)/solution.ts
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g' \
		templates/solution.test.template.ts > problems/$(PATTERN)/$(SLUG)/solution.test.ts
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g; s/{{PATTERN}}/$(PATTERN)/g; s/{{DIFFICULTY}}/$(DIFFICULTY)/g' \
		templates/solution.template.go > problems/$(PATTERN)/$(SLUG)/solution.go
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g' \
		templates/solution_test.template.go > problems/$(PATTERN)/$(SLUG)/solution_test.go
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g; s/{{PATTERN}}/$(PATTERN)/g; s/{{DIFFICULTY}}/$(DIFFICULTY)/g' \
		templates/solution.template.rs > problems/$(PATTERN)/$(SLUG)/solution.rs
	@sed 's/{{SLUG}}/$(SLUG)/g' \
		templates/cargo.template.toml > problems/$(PATTERN)/$(SLUG)/Cargo.toml
	@sed 's/{{TITLE}}/$(TITLE)/g; s/{{SLUG}}/$(SLUG)/g; s/{{PATTERN}}/$(PATTERN)/g; s/{{DIFFICULTY}}/$(DIFFICULTY)/g; s/{{DESCRIPTION}}/TODO: Add description/g' \
		templates/metadata.template.json > problems/$(PATTERN)/$(SLUG)/metadata.json
	@echo "problems/$(PATTERN)/$(SLUG)" >> Cargo.toml.tmp
	@if grep -q "^members = \[" Cargo.toml; then \
    sed '/^members = \[/a\
  "problems/$(PATTERN)/$(SLUG)",' Cargo.toml > Cargo.toml.new; \
    mv Cargo.toml.new Cargo.toml; \
  fi
	@rm -f Cargo.toml.tmp
	@echo "✓ Created problem structure at problems/$(PATTERN)/$(SLUG)/"
	@echo "✓ Added to Rust workspace"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Edit problem.md with the problem description"
	@echo "  2. Implement solution.ts, solution.go, and solution.rs"
	@echo "  3. Write tests in solution.test.ts, solution_test.go, and solution.rs"
	@echo "  4. Run 'make test' to verify all solutions pass"

list:
	@echo "Problems by Pattern:"
	@echo "===================="
	@for pattern in problems/*/; do \
		if [ -d "$$pattern" ]; then \
			pattern_name=$$(basename $$pattern); \
			echo ""; \
			echo "$$pattern_name:"; \
			for problem in $$pattern*/; do \
				if [ -d "$$problem" ]; then \
					problem_name=$$(basename $$problem); \
					if [ -f "$$problem/metadata.json" ]; then \
						title=$$(grep '"title"' "$$problem/metadata.json" | sed 's/.*: "\(.*\)".*/\1/'); \
						difficulty=$$(grep '"difficulty"' "$$problem/metadata.json" | sed 's/.*: "\(.*\)".*/\1/'); \
						echo "  - $$problem_name ($$title) [$$difficulty]"; \
					else \
						echo "  - $$problem_name"; \
					fi; \
				fi; \
			done; \
		fi; \
	done

validate:
	@echo "Validating repository structure..."
	@errors=0; \
	for pattern in problems/*/; do \
		if [ -d "$$pattern" ]; then \
			for problem in $$pattern*/; do \
				if [ -d "$$problem" ]; then \
					problem_path=$$(echo $$problem | sed 's:/*$$::'); \
					echo -n "Checking $$problem_path... "; \
					missing=""; \
					for file in problem.md solution.ts solution.test.ts solution.go solution_test.go solution.rs Cargo.toml metadata.json; do \
						if [ ! -f "$$problem_path/$$file" ]; then \
							missing="$$missing $$file"; \
						fi; \
					done; \
					if [ -n "$$missing" ]; then \
						echo "❌ Missing:$$missing"; \
						errors=$$((errors + 1)); \
					else \
						if ! python3 -m json.tool "$$problem_path/metadata.json" > /dev/null 2>&1; then \
							echo "❌ Invalid metadata.json"; \
							errors=$$((errors + 1)); \
						else \
							echo "✓"; \
						fi; \
					fi; \
				fi; \
			done; \
		fi; \
	done; \
	if [ $$errors -eq 0 ]; then \
		echo ""; \
		echo "✓ All problems validated successfully!"; \
	else \
		echo ""; \
		echo "❌ Found $$errors problem(s)"; \
		exit 1; \
	fi

test: test-ts test-go test-rs
	@echo ""
	@echo "✓ All tests passed!"

test-ts:
	@echo "Running TypeScript tests..."
	@if [ -n "$(PROBLEM)" ]; then \
		npm test -- problems/**/$(PROBLEM)/solution.test.ts; \
	else \
		npm test; \
	fi

test-go:
	@echo "Running Go tests..."
	@if [ -n "$(PROBLEM)" ]; then \
		go test ./problems/**/$(PROBLEM)/...; \
	else \
		go test ./problems/...; \
	fi

test-rs:
	@echo "Running Rust tests..."
	@if [ -n "$(PROBLEM)" ]; then \
		cargo test -p $(PROBLEM); \
	else \
		cargo test; \
	fi
