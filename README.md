# Flux Player

Flux Player is a high-performance, glassmorphic media suite for local and streaming content. Built from the ground up for desktop environments, it leverages the speed of Rust, the lightness of Svelte 5, and the power of Tauri 2.0.

![Flux Player Interface](docs/assets/flux.png)

## 🌟 Features

* **Cyber Dark Aesthetic:** A fully custom UI built from scratch using CSS Grid, absolute overlays, and a strict "Cyber Dark" theme (base: `#0a0a0c`, primary: `#8a2be2`, secondary: `#00ffff`).
* **Glassmorphism:** Elegant 1px translucent borders and high-key blur effects, ensuring a premium native application feel.
* **Dynamic Island:** A context-aware, animated UI element that morphs to provide playback notifications and state changes without ambient glows or box shadows.
* **Cinematic Detail View:** Enjoy immersive "Ken Burns" backdrop animations when viewing media details.
* **Robust Backend:** Powered by a Rust backend utilizing SQLite (`rusqlite`) for fast, secure, and parameterized database queries.
* **Smart Media Management:** Advanced playback state handling via a custom 'PlayerEngine' (headless Video.js implementation).

## 🚀 Getting Started

If you want to try Flux Player or contribute to its development, follow these steps:

### Prerequisites

* **Rust:** [Install Rust](https://www.rust-lang.org/tools/install) (1.75+)
* **Node.js:** [Install Node.js](https://nodejs.org/) (20+)
* **Linux Dependencies (If running on Linux):**
  ```bash
  sudo apt-get update
  sudo apt-get install -y libglib2.0-dev libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/xtarflex/flux-player.git
   cd flux-player
   cd flux-player
   ```

2. **Install frontend dependencies:**
   ```bash
   npm install
   ```

3. **Run the development server:**
   ```bash
   npm run tauri dev
   ```
   *(Note: The local development server runs on port 1420).*

## 📚 Documentation & Architecture

Flux Player is built with a deep commitment to architecture and design patterns. For detailed technical guides, explore the `docs/` folder in the root directory.

For comprehensive developer onboarding, backend instructions, and build processes, please see the [Developer README](flux-player/README.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) to get started.

## 📄 License

Flux Player is free and open-source software licensed under the [GPLv3 License](LICENSE).
