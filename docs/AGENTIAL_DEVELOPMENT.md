# Agential Collaboration & Development Guide

This document outlines how to transform Flux into an "AI-First" project, making it extremely easy for agents (like Antigravity) to build, debug, and maintain it without losing context.

## 1. The "Handover" Layer (Context)
Agents have a "context window." When you give them a roadmap, they don't have to waste tokens guessing.
- **docs/ENGINEERING_STANDARD.md**: The formal "Standard of Software Engineering" (Branching, Ready/Done states, Codestyle).
- **docs/SKILL_MAP.md**: High-priority index of localized Expertise Packs.
- **`.agent/agent.md`**: The mission statement. "Flux is a premium, animation-heavy media player. Every UI element must feel kinetic."
- **`DEV.log`**: A chronological record of *why* decisions were made.
- **`docs/ARCHITECTURE.md`**: The data "highway." How does a file on disk become a poster in the library?

## 2. The Elite Flux Pipeline (Review Cycle)
Flux uses a **Multi-Agent Review** system to ensure Beta-level quality. Every task follows this pipeline:

1.  **Work:** Executed via `.agents/workflows/workflow_active_coding.md`.
2.  **Initial Review:** Triggered via `.agents/workflows/workflow_initial_review.md`. A secondary agent audits the code.
3.  **Human Review:** The human developer performs the final sign-off.
4.  **Integrated Merge:** Executed via `.agents/workflows/workflow_merge_and_sync.md`.

## 3. The "Hands" (MCP - Model Context Protocol)
MCP is the bridge between the agent's brain and your system.
- **Context MCPs**: Allow agents to search your entire codebase, read git history, and analyze dependencies instantly.
- **Specialized MCPs**: You can build MCPs for the Tauri API or TMDB API so the agent can test real-time data fetching without you lifting a finger.

## 4. Agential Skills & Workflows
Skills are "Expertise Packs" for specific tasks.
- **`.agents/skills/`, `.agents/workflows/` **: Store `.md` files that describe "How to add a new UI component" or "How to handle a new media format." CHECK: `C:\Users\sunny\.agents\skills` to find a suite of skills for Antigravity.
- When an agent sees a workflow, they follow it perfectly, ensuring consistency with the **Flux Engineering Standard (FES)**.
- **Branch-Based Work:** Every task is executed in a unique branch and submitted for human review before merging.

## 5. Summary of Best Practices
1. **Explain the 'Why'**: If you want a specific animation curve, put it in the guide.
2. **Review Points**: Ask agents to "Stop and show me the plan" before they touch the code.
3. **Small Commits**: Encourage the agent to commit after every tiny win. It's our "undo" button.
4. **Knowledge Items (KIs)**: Every time we fix a big bug, we store the solution as a KI record.
