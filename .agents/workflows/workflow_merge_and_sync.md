---
description: MERGE-AND-SYNC (Final integration of a proven feature into main)
---

# WORKFLOW: MERGE & SYNC

**CRITICAL:** This workflow is triggered ONLY after a feature has been "proven and assessed" (Initial Agent Review + Human Review).

## Step 1: Pre-Merge Validation
1.  **Sync Local main:**
    ```bash
    git checkout main
    git pull origin main
    ```
2.  **Run Final Checks:**
    ```bash
    npm run check
    cargo check
    ```

## Step 2: Integration
1.  **Merge Feature:**
    ```bash
    git merge feature/task-name
    ```
2.  **Update CHANGELOG.md:**
    - Document the merge under the current Beta version.
    - Reference the `Rationale` from the `DEV.log`.

## Step 3: Forensic Closure
1.  **Propagate Version:** If this was a milestone, check if a version bump is required in `package.json`.
2.  **Final Push:**
    ```bash
    git push origin main
    ```

## Step 4: Cleanup
1.  **Branch Pruning:**
    ```bash
    git branch -d feature/task-name
    ```
2.  **Status Report:** Confirm to the human developer that the feature is now live on `main`.

## Step 5: Post-Task Sync
1.  **Checkout new main:** Ensure the environment is clean for the next task.
