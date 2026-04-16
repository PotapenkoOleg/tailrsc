---
name: wf-plan
description: >-
  Plan implementation for a GitHub issue. Reads the issue, researches official
  docs, and produces a TDD-based plan. Use when planning work, preparing an
  implementation, or creating a development plan for an issue.
argument-hint: GitHub issue URL or number
allowed-tools: Bash(gh issue view *) Bash(gh issue comment *) Bash(git *) WebFetch Read Glob Grep
disable-model-invocation: true
---

# Plan — Plan Implementation from a GitHub Issue

Read a GitHub issue and produce a development plan.

## Context

- Issue: `$ARGUMENTS`

## Steps

### 1. Read the Issue

Run `gh issue view $ARGUMENTS` to fetch the issue details. Then identify:
- The goal and expected outcome
- Technical requirements
- Potential challenges

### 2. Research

If needed, research using **official documentation only** (not blog posts or unofficial GitHub repos).

Use `WebFetch` to pull official docs when needed.

### 3. Raise Doubts

Point out any doubts about the implementation. **Only continue if you are confident about the approach.** If unsure, explain what's unclear and use the AskUserQuestion tool to ask questions.

### 4. Create the Plan

The plan must follow this structure:

1. **Create a working branch** with a descriptive slug
2. **Write tests first** (TDD)
3. **Write code** to make the tests pass
4. **Run tests** to verify
5. **Repeat** until all requirements are met
6. **Before committing**, ensure that:
   - Documentation is updated
   - README is updated

### 5. Present Options

Ask the user:

- **"Clear context and execute"** — start a fresh session to implement the plan
- **"Save plan as issue comment"** — post the plan as a comment on the issue for later
