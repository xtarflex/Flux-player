---
description: ACTIVE-CODING (Standard operating procedure for writing/modifying code)
---

# WORKFLOW: ACTIVE CODING

**CRITICAL:** You MUST execute these steps sequentially every single time you are tasked with writing, modifying, or debugging code in this repository. Do not skip steps.

## Step 1: Context Gathering (The Read Phase)

Before writing a single line of code, you must ingest the current project state and rules:

1. **Read Core Identity:** Briefly review `.agent/agent.md` to align with the core mission.

2. **Read Aesthetic Rules:** Check `docs/flux_master_guide.md` (and `.agents/skills/skill_aesthetic_enforcer.md` if available) to ensure you have the exact hex codes, fonts, and animation curves loaded into your context window.

3. **Read History:** Check `DEV.log` to see what was just completed and ensure you aren't repeating a recently fixed bug.

4. **Read Architecture:** Review `docs/ARCHITECTURE.md` or `docs/UX_JOURNEYS.md` specific to the feature you are touching.

## Step 2: Planning & Approval (The Think Phase)

1. **Draft the Plan:** Formulate a step-by-step technical plan of how you intend to build or fix the feature.

2. **Stop and Confirm:** Present this plan to the human developer. **Do not write the actual code yet.** Wait for the user to say "Approved" or "Proceed."

## Step 3: Execution (The Write Phase)

Once approved, begin writing the code:

1. **No Placeholders:** Write complete, functional code. Do not use `// TODO` or `/* insert logic here */`.

2. **Respect the Grid:** Ensure your layout adheres to the CSS Grid macro-architecture. Use absolute positioning ONLY for overlays.

3. **Native Mentality:** Remember this is a Tauri app. Leverage the Rust backend for heavy lifting (files, scraping, OS interactions) rather than trying to do it all in the browser DOM.

## Step 4: Verification (The Test Phase)

Before concluding your output, perform a self-audit:

1. **Did I use the right font?** (Syncopate/Outfit)

2. **Did I accidentally add an outer glow to the Dynamic Island?** (If yes, remove it. Crisp 1px border only).

3. **Did I respect the 0ms/500ms search latency rules?**

4. Prompt the human developer with specific manual verification steps (e.g., *"Please resize the window to ensure the Detail Panel drawer still slides smoothly."*).

## Step 5: Documentation (The Log Phase)

You must formally record your actions so the next agent (or your future self) has context:

1. **Update `DEV.log`:** Write a concise, chronological entry explaining *what* you changed, and more importantly, *why* you made specific architectural choices.

2. **Generate Knowledge Items (KIs):** If you solved a complex bug (like a race condition or a CSS layout thrashing issue), log it at the bottom of the `DEV.log` as a KI so it is never repeated.

3. **Propose Commit:** Provide the user with a suggested small, atomic Git commit message (e.g., `git commit -m "feat(ui): implement 0ms local search via css masking"`).