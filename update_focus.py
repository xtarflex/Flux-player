import os

def add_outline(filepath, css_class):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    target = f"{css_class}:focus-visible {{"
    replacement = f"{css_class}:focus-visible {{\n    outline: 2px solid var(--secondary);\n    outline-offset: 2px;"

    if target in content and "outline: 2px" not in content:
        content = content.replace(target, replacement)
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Added outline to {filepath}")
    else:
        # fallback for combined selectors
        target_combined = f"{css_class}:focus-visible {{\n    color: var(--text-main);"
        replacement_combined = f"{css_class}:focus-visible {{\n    color: var(--text-main);\n    outline: 2px solid var(--secondary);\n    outline-offset: 2px;"
        if target_combined in content:
            content = content.replace(target_combined, replacement_combined)
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Added outline (combined) to {filepath}")

for root, _, files in os.walk("flux-player/src/lib/components"):
    for file in files:
        if file.endswith(".svelte"):
            filepath = os.path.join(root, file)
            add_outline(filepath, ".icon-btn")
            add_outline(filepath, ".pill-btn")
            add_outline(filepath, ".icon-btn-large")
