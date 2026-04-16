---
name: skill-maker
description: Create a new Skill locally. Use when creating new skills, when the user wants to make a new skill, or when the user asks to turn recent work into a skill.
argument-hint: Your skill idea or context
---

# Skill Maker

Create a new Skill locally.

## Step 1: Fetch Specifications

**Always start here.** Fetch the official specs to ensure you create a valid skill:

```
WebFetch: https://agentskills.io/specification.md
WebFetch: https://code.claude.com/docs/en/skills.md
```

This gives you:
- Required/optional frontmatter fields
- Naming rules (lowercase alphanumeric + hyphens, 1-64 chars, no `--`, no leading/trailing `-`, must match parent directory name)
- Directory structure requirements
- Claude Code extensions (`argument-hint`, `disable-model-invocation`, etc.)

## Step 2: Understand the Skill Idea

The user will either:
- **Describe a skill idea** - ramble about what they want the skill to do
- **Reference recent work** - ask you to turn something you just worked on into a skill

Your job is to **infer** all the skill details from context:
- Skill name (following naming rules from spec)
- Description (what it does + when to use it with trigger keywords)
- Allowed tools (what tools the skill needs)
- Whether it should be user-invocable or model-invocable
- The actual skill instructions/content

Do NOT ask the user to fill in specific fields. Use your understanding of the conversation to create these.

## Step 3: Create the Skill

Create the skill directory **in the project** (`.claude/skills/`) by default. Only use the personal directory (`~/.claude/skills/`) if the user explicitly asks for a personal/global skill.

```bash
mkdir -p .claude/skills/<skill-name>
```

### Bundled resources convention

Skills use standard subfolders for bundled resources. Only create the ones that are needed:

| Subfolder | Purpose | Examples |
|-----------|---------|----------|
| `scripts/` | Executable code (Python, Bash, etc.) | `scripts/validate.sh`, `scripts/fetch_data.py` |
| `references/` | Documentation loaded into context as needed | `references/api_docs.md`, `references/patterns.md` |
| `assets/` | Files used in output, not loaded into context | `assets/template.html`, `assets/logo.png` |

**Prefer subfolders** for scripts, reference docs, and asset files. Small standalone files (like a `template.md` or `reference.md`) can live in the skill root, but use subfolders once you have several.

```bash
# Create only the subfolders the skill needs
mkdir -p .claude/skills/<skill-name>/scripts
mkdir -p .claude/skills/<skill-name>/references
mkdir -p .claude/skills/<skill-name>/assets
```

### Write the SKILL.md

```yaml
---
name: <skill-name>
description: <description with trigger keywords>
# Add other fields as appropriate:
# argument-hint: <hint>
# disable-model-invocation: true  # if user should invoke manually
# user-invocable: false           # hide from / menu, model-only
# model: sonnet                   # model override (sonnet, opus, haiku)
# context: fork                   # run in a subagent
# agent: <subagent-type>          # subagent type when using context: fork
# allowed-tools:
#   - <tool patterns>
---

# <Skill Title>

<Skill instructions and content>
```

**Guidelines (from spec):**
- Keep under 500 lines (use `references/` for detailed docs)
- Include step-by-step instructions
- Add examples of inputs/outputs
- Document common edge cases
- Reference all bundled resources so Claude knows they exist
- If invoking other skills, document which ones and when
- Use `$ARGUMENTS` in the body to reference the user's input (e.g., `Research the topic: $ARGUMENTS`)

## Step 4: Show Draft to User

Display the complete `SKILL.md` content to the user and ask them to review it.

Say something like:
> "Here's the draft for the `<skill-name>` skill. Please review it and let me know if it looks good or if you'd like any changes."

Wait for the user to confirm the skill looks good before proceeding.

## Step 5: Done - Report to User

Tell the user:
- Skill created at `.claude/skills/<skill-name>`
- They can now use it with `/<skill-name>`

## Example Usage

After working on something:
```
User: "Can you turn that into a skill?"
```

Or with an idea:
```
User: "I want a skill that helps me write release notes based on git commits"
```

Or directly:
```
/skill-maker
```
