---
name: wf-pr
description: >-
  Create a pull request from the current branch. Summarizes all commits and
  references connected issues. Use when creating a PR, staging changes, or
  opening a pull request.
argument-hint: "[target-branch] (defaults to main)"
allowed-tools: Bash(gh pr create *) Bash(git *)
disable-model-invocation: true
---

# PR — Create a Pull Request

Create a pull request from the current branch.

## Context

- Current branch: !`git branch --show-current`
- Commits on this branch: !`git log --oneline main..HEAD`
- Changed files: !`git diff main..HEAD --stat`

## Steps

### 1. Review All Commits

Look at **all** commits on this branch (not just the latest). Understand the full scope of changes.

### 2. Create the PR

Use `gh pr create` to create a pull request targeting `$ARGUMENTS` (default: `main`).

The PR should include:
- A clear, concise title
- A summary of what changed and why
- References to connected GitHub issues (e.g., `Closes #123`, `Fixes #45`)

Keep it short and focused — this is mechanical work, not a review.
