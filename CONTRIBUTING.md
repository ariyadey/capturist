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

To update the version across all files (`package.json`, `tauri.conf.json`, `Cargo.toml`, `snapcraft.yaml`), use the helper script:

```bash
npm run bump <new-version>
# Example: npm run bump 0.2.0
```

Then commit the changes and create a tag:

```bash
git commit -am "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags
```
