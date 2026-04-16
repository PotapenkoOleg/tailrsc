---
name: wf-publish
description: >-
  Merge a pull request and clean up branches and issues. Use when publishing,
  merging a PR, finalizing work, or cleaning up after a merge.
argument-hint: PR URL or number
allowed-tools: Bash(gh pr view *) Bash(gh pr merge *) Bash(gh issue close *) Bash(git *)
disable-model-invocation: true
---

# Publish — Merge PR and Clean Up

Merge an approved pull request and clean up Git.

## Context

- PR: `$ARGUMENTS`

## Steps

### 1. Check PR Status

Run `gh pr view $ARGUMENTS --json state,mergeable,mergeStateStatus` to check the PR status. If there are merge conflicts with the base branch, **stop immediately** and tell the user. Do not attempt to resolve conflicts automatically.

### 2. Merge

Perform the merge using a **merge commit** (not squash or rebase):

```bash
gh pr merge $ARGUMENTS --merge
```

### 3. Clean Up

After a successful merge:
- **Close related issues** referenced in the PR
- **Delete the working branch** (both remote and local)
- **Remove any obsolete branches** that are no longer needed

Do **not** review the PR — that has already been done.
