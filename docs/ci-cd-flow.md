# Capturist CI/CD Flow

> Companion reference for `.github/workflows/` and `.github/actions/`. Three thin trigger workflows delegate all packaging to one shared reusable workflow (`_package.yml`); only Snap `stable` promotion and Flathub repo updates are manual.

### 1. Open / Update a PR (before merge)

```mermaid
flowchart LR
    PR_IN["PR to main"] --> PR_CI["ci.yml\nFast checks only\n(lint, fmt, test, build, PR title, appstream)"]
    PR_CI --> PR_OUT(["Green checks on PR\nNO packaging, NO publish"])
```

### 2. Merge a normal PR into `main`

```mermaid
flowchart LR
    M_IN["Merged to main\n(normal commit)"] --> M_CI["ci.yml\nFast checks"]
    M_CI --> M_PKG["Calls _package.yml\nVersion: X.Y.Z+git.sha"]
    M_PKG --> M_OUT(["Publish Snap to 'edge'"])

    M_IN --> M_RP["release-please.yml\nOpens / updates Release PR"]
```

### 3. Merge the `release-please` PR (real release)

```mermaid
flowchart LR
    R_IN["Merge Release PR"] --> R_RP["release-please.yml\nCreates tag vX.Y.Z +\nGitHub Release"]
    R_IN -.-> R_SKIP["ci.yml: fast checks only\n(skips edge build)"]
    R_RP -->|"tag push fires release.yml"| R_PKG["release.yml\nCalls _package.yml\nVersion: X.Y.Z"]
    R_PKG --> R_OUT(["Upload 4 files to GH Release\n+ Publish Snap to 'candidate'\n+ Upload flathub-bundle artifact"])
    R_OUT -.->|"manual after smoke test"| R_STABLE(["Promote Snap -> 'stable'\nUpdate Flathub repo"])
```

### 4. Manual packaging test (on-demand, any branch)

```mermaid
flowchart LR
    MAN_IN["Actions -> Release\n-> Run workflow"] --> MAN_PKG["release.yml\nCalls _package.yml\nVersion: X.Y.Z+git.sha"]
    MAN_PKG --> MAN_OUT(["Download 4 artifacts\nNO publish"])
```

### Shared `_package.yml` (called by lanes 2, 3, and 4)

```mermaid
flowchart LR
    PKG_IN(["_package.yml\n(workflow_call)"]) --> P_TAURI["build-tauri (ubuntu-22.04)\n.deb + .rpm"]
    P_TAURI --> P_SNAP["build-snap (ubuntu-latest)\nwraps .deb + craftctl version\n.snap"]
    PKG_IN --> P_FLAT["build-flatpak (GNOME 50 container)\noffline sources + lint + .flatpak\n(+ flathub-bundle on v* tag)"]
```

## Workflow roles

| Workflow             | Trigger                                    | Purpose                                                                                                                                                                                                                                                       |
| -------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ci.yml`             | PR and `main` push                         | Fast quality gates (`frontend` lint/format/test/build, `backend` fmt/clippy/test, semantic PR title check, `appstreamcli`). On normal pushes to `main` (skipping `release-please` release commits), calls `_package.yml` and publishes `.snap` to **`edge`**. |
| `release-please.yml` | `main` push                                | Derives the next version from Conventional Commits, opens/maintains a release PR (`package.json`, `Cargo.toml`, `metainfo.xml`, `me.ariyadey.capturist.yml`), and on merge creates the `v*` tag + GitHub Release with release notes.                          |
| `release.yml`        | Manual `workflow_dispatch` **or** `v*` tag | Manual = calls `_package.yml` to build all 4 formats as downloadable artifacts (no publish). `v*` tag = calls `_package.yml` (with `flathub-bundle`), attaches all 4 formats to the GitHub Release, and publishes `.snap` to **`candidate`**.                 |
| `_package.yml`       | Reusable (`workflow_call`)                 | Single source of truth for building `.deb`, `.rpm`, `.snap` (version overridden via `craftctl` when `snap-version.txt` is present), and `.flatpak` (with offline sources + `flatpak-builder-lint`).                                                           |

## Shared composite actions

| Action                     | Role                                                                              |
| -------------------------- | --------------------------------------------------------------------------------- |
| `generate-flatpak-sources` | Regenerates offline `flatpak/cargo-sources.json` and `flatpak/node-sources.json`. |

## Snap Store channels

| Channel     | Content                                              | When                                                                           |
| ----------- | ---------------------------------------------------- | ------------------------------------------------------------------------------ |
| `edge`      | Every merged `main` commit, unique version per build | Automatic, **only on normal merge to `main`** (never on PRs)                   |
| `candidate` | Release candidate built from a `v*` tag              | On `v*` tag push                                                               |
| `stable`    | The tested release                                   | Manually promoted (`snapcraft promote capturist --from candidate --to stable`) |

## Release process (one sentence)

Optionally test packages via _Actions → Release → Run workflow_, merge Conventional-Commits PRs to `main`, merge the `release-please` PR (creates `v*` tag + GitHub Release + assets + Snap `candidate` + `flathub-bundle` artifact), then promote `candidate → stable` after smoke testing and copy `flathub-bundle` into the Flathub repo.

## Secrets used

- `SNAPCRAFT_STORE_CREDENTIALS` — Snap Store publishing (`edge`/`candidate`).
- `RELEASE_PLEASE_TOKEN` — a fine-grained PAT (`Contents: write`, `Pull requests: write`); required so the tag `release-please` creates can trigger `release.yml`.
