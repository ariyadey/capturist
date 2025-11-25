[![Release](https://github.com/ariyadey/capturist/actions/workflows/release.yml/badge.svg)](https://github.com/ariyadey/capturist/actions/workflows/release.yml)
[![capturist](https://snapcraft.io/capturist/badge.svg)](https://snapcraft.io/capturist)
[![capturist](https://snapcraft.io/capturist/trending.svg?name=0)](https://snapcraft.io/capturist)

# Capturist

<p style="text-align: center;">
  <strong>A sleek, minimal, and keyboard-driven desktop application for quickly adding tasks to <a href="https://todoist.com/">Todoist</a>.</strong>
</p>

<p style="text-align: center;">
  <img alt="Capturist Quick Add" src="public/img/screenshot-quick-add.png">
</p>

**Capturist** is designed for Linux and lives in your system tray. It can be summoned with a global shortcut, allowing you to capture tasks without breaking your workflow.

## ✨ Features

- **Global Shortcut:** Open the quick-add dialog from anywhere with a configurable global shortcut.
- **System Tray Icon:** Lives discreetly in your system tray for easy access.
- **Launch at Startup:** Automatically start with your system so it's always ready. (In Snap environment, you can manage it via system/Snap settings itself).
- **Native Notifications:** Get native desktop notifications confirming your task was added with a corresponding link to Todoist web app.
- **Secure:** Your Todoist token is securely stored in the operating system's native keyring.
- **Modern & Fast:** Built with Rust and Tauri for a small memory footprint and a snappy, native feel.
- **Familiar UI**: Based on Todoist User interface for a familiar look and feel.
- **Natural Language Detection**: [Todoist quick-add keywords](https://www.todoist.com/help/articles/use-task-quick-add-in-todoist-va4Lhpzz#h_01J1DEY59AGACAT8JW998EPH74) are supported.

## 🚀 Installation

Capturist is available as a Snap package from the Snap Store.

[![Get it from the Snap Store](https://snapcraft.io/en/light/install.svg)](https://snapcraft.io/capturist)

Or run the following command:

```bash
snap install capturist
```

Support for other packaging formats (like Flatpak, AppImage, .deb, .rpm, and AUR) may be added in the future based on community requests.

## 💻 Building from Source

Interested in contributing or running the latest development version? Here’s how to get started.

### Prerequisites

-   [Node.js and npm](https://nodejs.org/)
-   [Rust](https://www.rust-lang.org/tools/install)
-   [Angular CLI](https://angular.dev/cli): `npm install -g @angular/cli`
-   System dependencies for Tauri. Follow the [official Tauri guide](httpss://tauri.app/v1/guides/getting-started/prerequisites) for your OS.

### Development Mode

To run the application with hot-reloading for both the frontend and backend:

```bash
npm run tauri dev
```

### Production Build

To build and bundle the application for production:

```bash
npm run tauri build
```

The resulting binaries will be available in the `src-tauri/target/release` directory.

## 🛠️ Architecture & Technology

This project uses a modern stack to deliver a fast, native-like experience.

-   **Core Technologies:**
    -   **Frontend:** [Angular](https://angular.dev/) with [Angular Material](https://material.angular.dev/) and [Tailwind CSS](https://tailwindcss.com/).
    -   **Backend:** [Rust](https://www.rust-lang.org/) with the [Tauri](https://tauri.app/) framework.
-   **Key Conventions:**
    -   The Angular application is fully **zoneless** and uses **Signals** for state management, resulting in optimal performance.
    -   The backend handles the core OAuth2 flow and secure API interactions.
    -   Communication between the frontend and backend is done via Tauri's secure IPC (Commands and Events).
    -   Code quality is maintained with *Cargo Check*, *Prettier* and *ESLint*.

## 🛡️ Security

Security is a top priority for Capturist.

-   The OAuth2 `state` parameter (for CSRF protection) is generated and verified entirely on the backend.
-   To let Capturist securely save the Todoist API token in the OS native Keyring, you should allow it in sandboxed environments:
    - **Snap**: Run `snap connect capturist:password-manager-service`
-   A strict Content Security Policy (CSP) is configured in `tauri.conf.json` to mitigate cross-site scripting (XSS) risks.

## 🤝 Contributing

Pull requests are welcome!
For major changes, please open an issue first to discuss what you would like to change.
Please ensure to add/update tests as appropriate.

Before submitting, please run `npm run lint` to ensure your code adheres to the project's style.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

> _Capturist is an unofficial Todoist client and is not affiliated with or endorsed by Doist._
