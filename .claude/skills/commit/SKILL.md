---
name: commit
description: >-
  Create git commit(s) using Conventional Commits. Groups changes by type
  and auto-detects GitHub issue references from branch names. Use when
  committing, saving changes, or when the user says "commit".
argument-hint: "[single]"
allowed-tools: Bash(git *)
---

# Git Commit

Create well-structured commits following the [Conventional Commits](https://conventionalcommits.org) standard.

## Context

- Current git status: !`git status`
- Staged changes: !`git diff --staged`
- Unstaged changes: !`git diff`
- Current branch: !`git branch --show-current`
- Recent commits (for context, NOT as a style guide): !`git log --oneline -10`

## Arguments

- **`single`** (optional): Create ONE commit for all changes instead of logical groupings
  - `/commit single` -> one commit with all changes
  - `/commit` (default) -> multiple logical commits grouped by type

## Steps

### 1. Check for GitHub Issue Context

Look at the current branch name for issue numbers (e.g. `issue-123`, `fix-45`, `feature/12`). If found, include the appropriate reference in commit messages:
- `fixes #123` — bug fixes that close the issue
- `closes #123` — features that complete the issue
- `refs #123` — partial work or related changes

### 2. Analyze and Group Changes

**If `single` argument:** Stage everything and create one commit.

**If no argument (default):** Group changes by type and create separate commits:

| Type | Use for |
|------|---------|
| `feat` | New functionality or capabilities |
| `fix` | Bug fixes or corrections |
| `refactor` | Code restructuring without behavior change |
| `docs` | Documentation updates |
| `test` | Test additions or updates |
| `build` | Build system, dependencies, config changes |
| `style` | Formatting, linting fixes |
| `chore` | Maintenance tasks |

If unsure which type fits, consult [references/conventional-commits.md](references/conventional-commits.md) for detailed definitions, examples per type, and common judgment calls.

### 3. Write Commit Messages

Format: `type(scope): description`

- **Scope is optional.** Use it when the change targets a clear module (e.g. `auth`, `api`). Omit it for broad or small changes.
- **Body is optional.** Add one only when the "why" isn't obvious from the first line. Skip it for simple commits.

Rules:
- Always use a type from the table above — ignore the style of existing commits
- First line under 72 characters
- Imperative mood ("add" not "added")
- Body explains **why**, not what
- Include issue reference when applicable

Examples:

```
feat(auth): add password reset flow

- Add reset token generation endpoint
- Create email template for reset link
- Add token expiry validation

Closes #42
```

```
fix(api): handle empty response from payment provider

Fixes #78
```

```
docs: update installation instructions
```

### 4. Stage and Commit

Stage files explicitly by path. Use a HEREDOC for multiline messages:

```bash
git add src/auth/reset.py src/auth/tests/test_reset.py
git commit -m "$(cat <<'EOF'
feat(auth): add password reset flow

- Add reset token generation endpoint
- Create email template for reset link

Closes #42
EOF
)"
```

Never use `git add -A` or `git add .` — always stage files by name.
