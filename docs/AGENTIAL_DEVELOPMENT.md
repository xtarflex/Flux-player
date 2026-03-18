# Agential Collaboration & Development Guide

This document outlines how to transform Flux into an "AI-First" project, making it extremely easy for agents (like Antigravity) to build, debug, and maintain it without losing context.

## 1. The "Handover" Layer (Context)
Agents have a "context window." When you give them a roadmap, they don't have to waste tokens guessing.
- **`.agent/agent.md`**: The mission statement. "Flux is a premium, animation-heavy media player. Every UI element must feel kinetic."
- **`DEV.log`**: A chronological record of *why* decisions were made. (e.g., "Reset to 8218db7 because surgical fixes couldn't solve the grid-shifter ghost.")
- **`docs/ARCHITECTURE.md`**: The data "highway." How does a file on disk become a poster in the library?

## 2. The "Eyes" (Verification)
Agents need a way to see if they broke something. 
- **Automated Tests (Vitest/Playwright)**: An agent can run `npm test`. If it passes, they know the core logic is safe.
- **Verification Checklists**: Manual steps the agent should ask the user to perform (e.g., "Resize the window to 800px and verify the sidebar collapse").

## 3. The "Hands" (MCP - Model Context Protocol)
MCP is the bridge between the agent's brain and your system.
- **Context MCPs**: Allow agents to search your entire codebase, read git history, and analyze dependencies instantly.
- **Specialized MCPs**: You can build MCPs for the Tauri API or TMDB API so the agent can test real-time data fetching without you lifting a finger.

## 4. Agential Skills & Workflows
Skills are "Expertise Packs" for specific tasks.
- **`.agents/workflows/`**: Store `.md` files that describe "How to add a new UI component" or "How to handle a new media format." 
- When an agent sees a workflow, they follow it perfectly, ensuring consistency with your coding style.

## 5. Summary of Best Practices
1. **Explain the 'Why'**: If you want a specific animation curve, put it in the guide.
2. **Review Points**: Ask agents to "Stop and show me the plan" before they touch the code.
3. **Small Commits**: Encourage the agent to commit after every tiny win. It's our "undo" button.
4. **Knowledge Items (KIs)**: Every time we fix a big bug, we store the solution as a KI record.
