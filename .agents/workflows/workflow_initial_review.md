---
description: INITIAL-REVIEW (Pre-flight audit of a feature branch before human review)
---

# WORKFLOW: INITIAL AGENT REVIEW

**CRITICAL:** This workflow MUST be performed by an independent agent (or a secondary "persona" pass) before a task is submitted to the human developer.

## Step 1: Ingest Review Context
1.  **Load Skill:** Ingest `c:\Users\sunny\.agents\skills\code-reviewer`.
2.  **Read Target:** Read the technical plan and the changes made in the target branch.
3.  **Read Standards:** Load `docs/ENGINEERING_STANDARD.md` and `docs/forensic_blueprint.md`.

## Step 2: Technical Audit
Evaluate the code against these categories:

### 1. Correctness & Quality
- Does the code actually solve the problem described in the plan?
- Are there any `any` types in TS or `unwrap()` calls in Rust?
- Are JSDoc comments present and accurate?

### 2. Forensic Blueprint Compliance
- **Performance:** Does this use an expensive DOM approach where a canvas/Svelte trick would be faster?
- **Aesthetics:** Are the 1px borders and spring physics strictly followed?
- **Modularity:** Are the Svelte files too large? (>300 lines should be split).

### 3. Security & Safety
- Are external inputs (file paths, TMDB keys) properly sanitized?
- Are Rust commands guarded against malicious input?

## Step 3: Generation of Report

Produce an **Initial Review Report** artifact with the following structure:

- **Verdict:** [APPROVE | REQUEST CHANGES | BLOCK]
- **Critical Fixes:** Any non-negotiable issues (panics, logic bugs).
- **Aesthetic Suggestions:** Minor UI tweaks to reach "Flux Elite" quality.
- **Performance Notes:** Potential frame-rate optimizations.

## Step 4: Verification
1.  If verdict is **APPROVE**: Notify the lead agent to proceed to Human Review.
2.  If verdict is **REQUEST CHANGES**: Return to the lead agent with the fixes.
