---
description: PUBLISH-RELEASE (Automated semantic versioning, tagging, and CI binary publishing)
---

# WORKFLOW: PUBLISH RELEASE

**CRITICAL:** You MUST execute these steps sequentially when the user requests a new version of Flux Player. This ensures absolute stability in the release pipeline and artifact delivery.

## Step 1: Pre-flight Audit (The Sync Phase)

1. **Verify Workspace Purity**:
   - Run `git status` to ensure the working directory is clean.
   - Run `git fetch origin` to update remote tracking.
   - Ensure the current branch is `main`.

2. **Discover Current Version**:
   - Read `flux-player/package.json` to find the `"version"` field.
   - Read `flux-player/src-tauri/Cargo.toml` to ensure the version matches.

3. **Propose the Increment**:
   - Identify the next logical version (e.g., `0.2.0-beta.1` → `0.2.0-beta.2`).
   - Present the current version and the target release version to the user for explicit confirmation.

## Step 2: Meta-Sync (The Bump Phase)

1. **Synchronize Files**:
   - If the version in `package.json` or `Cargo.toml` is outdated, use `replace_file_content` to update them to the target version.
   - Ensure all branding (README, Titlebars) reflects the new release state if necessary.

2. **Commit the Bump**:
   - Run `git add .`
   - Run `git commit -m "chore: release v[VERSION]"`

## Step 3: Launch (The Push Phase)

1. **Create the Tag**:
   - Run `git tag v[VERSION]` (Example: `git tag v0.2.0-beta.2`).

2. **Push to Remote**:
   - Run `git push origin main` (Update the code).
   - Run `git push origin v[VERSION]` (Trigger the Release CI).

## Step 4: Verification (The Monitor Phase)

1. **Watch the Builder**:
   - Run `gh run list --limit 1` to find the tag-triggered CI run.
   - Wait for the `build-windows` job to reach success.

2. **Confirm Artifact Delivery**:
   - Once the `release` job completes, run `gh release view v[VERSION]` to ensure the Windows installer (`.exe`) is attached and the release is marked as a **Pre-release**.

3. **Final Handover**:
   - Provide the user with the direct URL to the GitHub Release.
   - Update `DEV.log` with the launch entry.
