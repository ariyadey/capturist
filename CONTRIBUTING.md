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

### Linting & Formatting

To ensure your code passes CI checks, please run the following before pushing:

```bash
# Frontend
npm run lint
npm run format:check

# Backend (Rust)
cd src-tauri
cargo fmt --check
cargo clippy -- -D warnings
```

### Release Process

- **Edge**: Every push to `main` is automatically built and published to the Snap Store `edge` channel.
- **Stable**: Stable releases are triggered manually via the **CI/CD** workflow. This will:
  - Bump the version in all configuration files.
  - Build the application (Deb, RPM, Snap, Flatpak).
  - Create a git tag (e.g., `v1.0.0`) and push it.
  - Create a GitHub Release with all artifacts attached.
  - Publish to the Snap Store `stable` channel.

### Flatpak & Flathub

To publish to Flathub:

1.  The `release.yml` workflow builds a `capturist.flatpak` bundle.
2.  For the **first release**, you must manually submit the manifest (`me.ariyadey.capturist.yml`) to [Flathub](https://github.com/flathub/flathub).
3.  For subsequent releases, you will update the Flathub repository with the new version.

### Versioning & Releasing

You can release either locally or via the Cloud (GitHub Actions).

#### Option 1: Cloud Release (Recommended)

1.  Go to **Actions** > **CI/CD**.
2.  Click **Run workflow**.
3.  Enter the new version number (e.g., `0.2.6`).
4.  (Optional) Enter release notes for AppStream metadata.
5.  Click **Run workflow**.

The workflow will automatically:

- Bump the version.
- Build all artifacts.
- **Only if the build succeeds**: Commit the bump, create the tag, and publish the release.

#### Option 2: Local Release

If you prefer to release locally, you must manually update the version in:

- `package.json`
- `src-tauri/Cargo.toml`
- `me.ariyadey.capturist.metainfo.xml`

Then commit the changes and create a tag:

```bash
git commit -am "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags
```
