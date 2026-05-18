import os

filepath = "flux-player/src/lib/components/footer/RightActions.svelte"
with open(filepath, 'r', encoding='utf-8') as f:
    content = f.read()

target = ".icon-btn-large:not(:disabled):hover {"
replacement = ".icon-btn-large:not(:disabled):hover,\n  .icon-btn-large:focus-visible {\n    outline: 2px solid var(--secondary);\n    outline-offset: 2px;"

if target in content:
    content = content.replace(target, replacement)
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"Added outline to {filepath}")
