# Flux Player: Directory Structure

This project enforces a strict separation between human-readable architectural blueprints (`/docs`) and AI-executable instructions (`/.agents`). 

```text
flux/
├── docs/                               # 📂 The Master Blueprints (Human & AI Context)
│   ├── PROJECT_CHARTER.md              # The roadmap, scope, and core vision
│   ├── UX_JOURNEYS.md                  # Step-by-step user flows and edge cases
│   ├── SETTINGS_SPEC.md                # Specifications for the settings hub
│   ├── deep_logic_supplement.md        # Technical execution for hidden logic
│   ├── forensic_blueprint.md           # Explanations of the "magic" UI features
│   └── DIRECTORY_STRUCTURE.md          # This file
│
├── .agents/                            # 🤖 The AI Brain (Hidden from standard view)
│   ├── agent.md                        # The core mission and identity for the AI
│   ├── skills/
│   │   └── skill_aesthetic_enforcer.md # Strict CSS/UI rules for the AI to follow
│   └── workflows/
│       ├── workflow_reconstruct.md     # Step-by-step dev instructions for initial build
│       └── workflow_active_coding.md   # The 5-step loop for all daily coding tasks
│
├── src/                                # 💻 SvelteKit frontend code (UI/Components)
├── src-tauri/                          # 🦀 Rust backend code (Filesystem/Scrapers)
└── package.json                        # 📦 Project dependencies