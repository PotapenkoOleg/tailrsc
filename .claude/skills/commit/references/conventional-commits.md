# Conventional Commits Type Guide

Full reference: https://conventionalcommits.org

Use this when you're unsure which type fits a change.

## Type Definitions

### `feat` — New functionality

A new capability that didn't exist before. The user can do something new.

```
feat(search): add full-text search across all documents
feat: add dark mode toggle
feat(api): expose bulk import endpoint
```

**Not `feat`:** Improving an existing feature's performance (that's `refactor` or `fix`).

### `fix` — Bug fixes

Something was broken and now it works correctly.

```
fix(auth): prevent session expiry during active use
fix: correct off-by-one error in pagination
fix(export): handle UTF-8 filenames on Windows
```

**Not `fix`:** Changing behavior that was working as designed but is now unwanted (that's `feat` or `refactor`).

### `refactor` — Restructuring without behavior change

The code changes, but the user sees no difference. Tests should still pass without modification.

```
refactor(db): replace raw SQL with query builder
refactor: extract validation logic into shared module
refactor(auth): simplify token refresh flow
```

**Not `refactor`:** If you also fixed a bug or changed behavior, use `fix` or `feat` instead.

### `docs` — Documentation only

README, comments, docstrings, API docs, guides. No code behavior changes.

```
docs: update installation instructions for Python 3.12
docs(api): add examples to authentication section
docs: fix broken links in contributing guide
```

### `test` — Test changes only

Adding, updating, or fixing tests. No production code changes.

```
test(auth): add edge case tests for expired tokens
test: increase coverage for payment module
test(api): fix flaky integration test
```

### `build` — Build system and dependencies

Changes to how the project builds, its dependencies, or its tooling config.

```
build: upgrade Django from 4.2 to 5.0
build(docker): optimize image size with multi-stage build
build: add pre-commit hook configuration
```

### `style` — Formatting only

Whitespace, semicolons, linting fixes. No logic changes.

```
style: apply Black formatting to all Python files
style: fix ESLint warnings in components/
style: normalize line endings to LF
```

**Not `style`:** CSS changes that affect what the user sees are `feat` or `fix`.

### `chore` — Maintenance

Everything else that doesn't fit above. Dependency bumps, CI tweaks, gitignore updates.

```
chore: update .gitignore for new IDE
chore(ci): add Python 3.13 to test matrix
chore: remove unused environment variables
```

## Common Judgment Calls

| Situation | Type | Why |
|-----------|------|-----|
| Fix a typo in code output | `fix` | User-visible behavior changed |
| Fix a typo in a comment | `docs` | No behavior change |
| Rename a variable for clarity | `refactor` | Code change, no behavior change |
| Add input validation | `feat` | New capability (rejecting bad input) |
| Fix missing input validation | `fix` | It should have been there |
| Update a dependency for security | `fix` | Addresses a vulnerability |
| Update a dependency for features | `build` | Tooling change |
| Delete dead code | `refactor` | Code restructuring |
| Move files to new directory | `refactor` | Code restructuring |
