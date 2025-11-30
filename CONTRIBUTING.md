# Contributing to Capturist

Thank you for your interest in contributing to Capturist!

## VCS Strategy

We follow a **Trunk-Based Development** model (GitHub Flow).

### Branching

- **`main`**: The default branch. Always stable and deployable.
- **Feature Branches**: Create a new branch for every feature or fix.
  - Naming convention: Use descriptive names (e.g., `add-login-feature`, `fix-auth-bug`).

### Commits

Write clear and concise commit messages.

- Start with a short summary (under 50 characters).
- Use the imperative mood (e.g., "Add feature" not "Added feature").
- If necessary, add a more detailed description in the body.

### Release Process

- **Edge**: Every push to `main` is automatically built and published to the Snap Store `edge` channel.
- **Stable**: Stable releases are triggered by pushing a git tag (e.g., `v1.0.0`). This will:
  - Build the application.
  - Create a GitHub Release.
  - Publish an AppImage
  - Publish to the Snap Store `stable` channel.
  - Build a Flatpak bundle (`.flatpak`) and attach it to the release.

### Flatpak & Flathub

To publish to Flathub:
1.  The `release.yml` workflow builds a `capturist.flatpak` bundle.
2.  For the **first release**, you must manually submit the manifest (`distribution/flatpak/me.ariyadey.capturist.yml`) to [Flathub](https://github.com/flathub/flathub).
3.  For subsequent releases, you will update the Flathub repository with the new version.

### Versioning & Releasing

You can release either locally or via the Cloud (GitHub Actions).

#### Option 1: Cloud Release (Recommended)

1.  **Bump Version**:
    - Go to **Actions** > **Bump Version**.
    - Run the workflow with the new version (e.g., `0.2.2`).
    - This pushes a new tag `v0.2.2`.

2.  **Trigger Release**:
    - Go to **Actions** > **Release**.
    - Run the workflow, selecting the **Tag** you just created (e.g., `v0.2.2`).

#### Option 2: Local Release

If you prefer to release locally, you must manually update the version in:
-   `package.json`
-   `src-tauri/tauri.conf.json`
-   `src-tauri/Cargo.toml`
-   `snapcraft.yaml`

Then commit the changes and create a tag:

```bash
git commit -am "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags
```
