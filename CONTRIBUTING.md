# Contributing to Capturist

Thank you for your interest in contributing to Capturist!

## VCS Strategy

We follow a **Trunk-Based Development** model (GitHub Flow).

### Branching

- **`main`**: The default branch. Always stable and deployable.
- **Feature Branches**: Create a new branch for every feature or fix.
  - Naming convention: Use descriptive names (e.g., `add-login-feature`, `fix-auth-bug`).

### Commits

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]
```

- Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `build`, `ci`, `perf`.
  Prefix breaking changes with `!` (e.g., `feat!: drop X`).
- Examples: `fix(quick-add): handle blur event`, `feat(snap): unique edge versions`, `chore: bump deps`.
- Pull request titles are validated against Conventional Commits in CI (`action-semantic-pull-request`).
- Write a short summary (under 50 characters) in the imperative mood; add details in the body only when they explain the _why_.

### Pull Requests

- Open a PR for every change; `main` is protected.
- Keep PRs small and focused on one logical change. Squash-merge with a Conventional
  Commit title so the changelog stays readable.
- Run the gates below before pushing.

### Linting, Formatting & Tests

```bash
# Frontend
npm run lint
npm run format:check
npm test
npm run build

# Backend (Rust)
cd src-tauri
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## CI/CD Pipelines

> 📊 Full flow diagram: [docs/ci-cd-flow.md](docs/ci-cd-flow.md)

- **CI** (`.github/workflows/ci.yml`) — runs on every PR and push to `main`:
  fast quality gates (`frontend` lint/format/test/build, `backend` fmt/clippy/test,
  semantic PR title check, and `appstreamcli` validation). PRs never package or publish.
  On normal pushes to `main` (skipping `release-please` release commits), CI calls
  `_package.yml` with `<version>+git.<sha>` and publishes the snap to the Snap Store
  **`edge`** channel.
- **Package** (`.github/workflows/_package.yml`) — reusable workflow shared by `ci.yml`
  and `release.yml`. Builds `.deb`/`.rpm` on `ubuntu-22.04`, wraps the `.deb` into a
  `.snap` (overriding the version via `craftctl set version` using `snap-version.txt`),
  and builds + lints `.flatpak` inside the GNOME 50 container.
- **Release** (`.github/workflows/release.yml`) — two modes:
  - **Manual run (`workflow_dispatch`)**: calls `_package.yml` on any branch to build
    `.deb`, `.rpm`, `.snap`, and `.flatpak` as downloadable workflow artifacts. Nothing
    is published. Use this to test packaging on a PR branch or `main` before releasing.
  - **Tag push (`v*`)**: calls `_package.yml` with the tag version (and uploads a
    `flathub-bundle` workflow artifact), attaches all four formats to the GitHub Release
    created by `release-please`, and publishes the snap to the Snap Store **`candidate`**
    channel.
- **release-please** (`.github/workflows/release-please.yml`) — watches `main`,
  derives the next version from Conventional Commits, and opens/maintains a release
  pull request that bumps `package.json`, `src-tauri/Cargo.toml`, the single AppStream
  metainfo `<release>` entry (`x-release-please-version-date` / `x-release-please-version`),
  and `me.ariyadey.capturist.yml` (`x-release-please-version`), and (when merged) creates
  the `v*` tag and a GitHub Release with auto-generated release notes (`skip-changelog: true`
  keeps no `CHANGELOG.md` file in the repo).
- **AppStream metainfo** — `me.ariyadey.capturist.metainfo.xml` holds a single `<release>`
  entry with a `<url type="details">` link to the GitHub Release (release history lives on
  GitHub Releases). It is updated solely by `release-please` and never mutated at build time.

### Snap Store channels

| Channel     | Content                                              | When              |
| ----------- | ---------------------------------------------------- | ----------------- |
| `edge`      | Every merged `main` commit, unique version per build | Automatic         |
| `candidate` | The release candidate built from a `v*` tag          | On tag push       |
| `stable`    | The tested release                                   | Manually promoted |

After smoke-testing the `candidate` snap, promote it to `stable`:

```bash
snapcraft promote capturist --from candidate --to stable
```

### Release Process (cloud)

Releases are driven entirely by **release-please**; no local tooling is required.

1. **Test a packaged build without releasing**: Actions > Release > Run workflow.
   Download the `.deb`/`.rpm`/`.snap`/`.flatpak` artifacts from the run and test them.
2. **Merge code to `main`** with Conventional Commit PR titles. `release-please` watches
   `main`, derives the next version from your commits (e.g. `fix` → patch, `feat` →
   minor) and opens a **release pull request** that bumps versions and prepares release notes.
3. **Review and merge** the release PR.
4. On merge, `release-please` creates the tag `vX.Y.Z` and a GitHub Release with the
   changelog notes. The tag push triggers the Release workflow, which builds all four
   formats, attaches them to the release, uploads the `flathub-bundle` artifact, and
   publishes the snap to **`candidate`**.
5. **Promote to stable** after smoke-testing `candidate`:

   ```bash
   snapcraft promote capturist --from candidate --to stable
   ```

> **Setup note**: the `release-please.yml` workflow uses the
> `RELEASE_PLEASE_TOKEN` secret — a fine-grained PAT with **Contents: write** and
> **Pull requests: write** on this repo. The default `GITHUB_TOKEN` cannot re-trigger
> workflows, so without a PAT the tag `release-please` creates would never fire the
> Release workflow.

### Flatpak & Flathub

1. The Release workflow builds a `capturist_<version>_amd64.flatpak` bundle and attaches it to the GitHub Release, and uploads a `flathub-bundle` workflow artifact containing `me.ariyadey.capturist.yml` (with both `tag` and `commit` pinned), `flatpak/cargo-sources.json`, and `flatpak/node-sources.json`.
2. For the **first release**, submit the manifest and offline sources from `flathub-bundle` to [Flathub](https://github.com/flathub/flathub).
3. For subsequent releases, download `flathub-bundle` from the `v*` Release workflow run and copy those 3 files into the Flathub repository.

## Engineering invariants

These behaviors are load-bearing; changing them needs care and tests:

- **Linux deep links:** a `capturist://` URL reaches an already-running app only through single-instance DBus `argv` forwarding. OAuth redirects are handled in `on_another_instance_trial` (`argv` parse → `authenticate()`), not `on_open_url`.
- **Snap DBus name:** the declared plug/slot name must match the pinned `tauri-plugin-single-instance` runtime name (`me.ariyadey.capturist.SingleInstance` for v2.4.3).
- **OAuth:** Todoist is a PKCE public client (no secret); access tokens expire in 1 hour; refresh tokens rotate. Never replay a refresh token, and keep concurrent refreshes deduplicated.
- **Angular bundle:** `undici`, `fs`, and `path` stay in `angular.json` `externalDependencies` so esbuild does not bundle Node-only Todoist SDK imports.
- **Flatpak:** `_package.yml` pins the manifest `capturist` git source to the build commit (`commit:` for dev builds; `tag:` + `commit:` for `v*` releases) and regenerates offline `node-sources.json`/`cargo-sources.json` from the checked-out lockfiles.
