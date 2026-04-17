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

4. **Search Skill Library:** Scan `c:\Users\sunny\.agents\skills` for task-specific expertise (e.g., `git`, `senior-backend`, `ui-design-system`). **Explicitly state which skills you have loaded in your plan.**

5. **Read Architecture & Standards:** Review `docs/ARCHITECTURE.md`, `docs/UX_JOURNEYS.md`, and **strictly** align with `docs/ENGINEERING_STANDARD.md`.

6. **Verify Blueprint:** Check `docs/forensic_blueprint.md` for any performance "magic" required for this feature.

## Step 2: Isolation & Planning (The Think Phase)

1. **Create Branch:** Immediately checkout a new feature branch: `git checkout -b feat/task-name` or `fix/task-name`.

2. **Draft the Plan:** Formulate a step-by-step technical plan of how you intend to build or fix the feature, including an ADR if architectural changes are required.

3. **Stop and Confirm:** Present this plan to the human developer. **Do not write the actual code yet.** Wait for the user to say "Approved" or "Proceed."

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

5. **Premium UX Audit:** Conduct a final check against the `ENGINEERING_STANDARD.md` §5 (Sleight of Hand) to ensure 60fps compliance.

## Step 5: Propose & Document (The Log Phase)

1. **Automated Push:** Execute `git add .` and `git commit -m "feat/fix: [description]"` followed by `git push origin [branch-name]`.

2. **Propose Initial Review:** Notify the human developer that the branch is ready and trigger the `workflow_initial_review.md` for another agent to audit.

3. **Update `DEV.log`:** Write a concise, chronological entry explaining *what* you changed, and more importantly, the *Rationale* behind specific architectural choices.

3. **Generate Knowledge Items (KIs):** If you solved a complex bug (like a race condition or a CSS layout thrashing issue), log it at the bottom of the `DEV.log` as a KI so it is never repeated.

4. **Suggest Commit:** Provide the user with a suggested small, atomic Git commit message (e.g., `git commit -m "feat(ui): implement 0ms local search via css masking"`).