# AGENTS.md — Capturist

**Capturist** is a Linux desktop app (Rust + Tauri backend, Angular frontend) for quickly adding tasks to Todoist via a global shortcut and system tray.

## Project structure

- `src/` — Angular frontend (zoneless, Signals-based, Angular Material + Tailwind CSS).
  - `src/app/` — feature components (`landing-page/`, `quick-add/`), `shared/` (IPC, Todoist API, theme).
  - `src/index.html`, `src/main.ts`, `src/styles.scss` — app wiring and global styles.
- `src-tauri/` — Rust backend (Tauri v2).
  - `src/desktop/` — tray, shortcut, autostart, window, cli, update.
  - `src/ipc/` — Tauri commands, events, deep links.
  - `src/external/todoist/` — OAuth2 + Todoist API integration.
  - `src/shared/` — state, error, environment, storage (keyring).
- `me.ariyadey.capturist.yml` / `me.ariyadey.capturist.metainfo.xml` / `snapcraft.yaml` — desktop packaging metadata (`flatpak/` offline sources are generated at CI build time).
- Tests are **co-located** as `*.spec.ts` files next to their source (e.g. `quick-add-dialog.spec.ts`).

## Build, test & development commands

```bash
npm install          # install frontend deps
npm run start        # Angular dev server (hot reload)
npm run build        # production Angular build
npm run lint         # ESLint on the frontend
npm run format:check # Prettier formatting check
npm run tauri dev    # run the full desktop app (hot reload both sides)
npm run tauri build  # production bundle (see src-tauri/target/release)
# Backend (Rust), run from src-tauri/
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Code style & naming conventions

- Format with **Prettier** (`.prettierrc.yml` — organizes imports, attributes, Tailwind classes) and **ESLint**; these run in CI.
- 2-space indentation, single quotes, no semicolons (Perl/prettier defaults); follow `.editorconfig`.
- Angular components use the `.spec.ts` suffix for unit tests (Vitest + jsdom via `@angular/build:unit-test`), kept next to the source.
- Backend Rust is formatted with `rustfmt` and must pass `cargo clippy -- -D warnings`; add `#[cfg(test)]` unit tests where behavior warrants.
- The Angular app is **zoneless** — use Signals for state, avoid Zone.js-dependent patterns.

## VCS: commits & pull requests

- **Trunk-based / GitHub Flow**: `main` is always stable; create a feature branch per change (e.g. `fix-tray-icon`, `add-autostart-perm`).
- Commit/PR titles follow **Conventional Commits** (`feat:`, `fix(scope):`, `refactor:`, `docs:`, `chore:`, ...); PR titles are enforced by `action-semantic-pull-request` in CI.
- Dependabot handles dependency bumps in dedicated PRs (prefixed with `chore`).
- Before submitting: run `npm run lint`, `npm run format:check`, `npm test`, `npm run build` (frontend) and `cargo fmt --check && cargo clippy -- -D warnings && cargo test` (backend).
- Open an issue to discuss major changes before opening a PR.
- Always add/update unit tests for every bug fix, feature, and enhancement to keep the app bulletproof.
- Always set appropriate labels, a GitHub `type` (e.g. `Bug`, `Feature`, `Task`), and assign PRs to `ariyadey` (use `gh api -X PATCH repos/ariyadey/capturist/issues/<number>` if `gh pr edit` fails on `projectCards`).
- Persist cross-session project rules, conventions, and significant architectural decisions in this file (`AGENTS.md`).

## CI/CD

- `.github/workflows/ci.yml` — PRs and `main` pushes: fast quality gates (`frontend` lint/format/test/build, `backend` fmt/clippy/test, semantic PR title check, `appstreamcli`). PRs never package or publish. On normal `main` pushes (skipping `release-please` release commits), calls `_package.yml` with `<version>+git.<sha>` and publishes `.snap` to Snap Store `edge`.
- `.github/workflows/_package.yml` — reusable `workflow_call` shared by `ci.yml` and `release.yml`: builds `.deb`/`.rpm` (`ubuntu-22.04`), `.snap` (via `snap-version.txt` + `craftctl set version`), and `.flatpak` (GNOME 50 container with `generate-flatpak-sources` + `flatpak-builder-lint`).
- `.github/workflows/release-please.yml` — watches `main`, derives the next version from Conventional Commits, and opens/maintains a release PR (bumps `package.json`, `Cargo.toml`, the single AppStream metainfo `<release>` entry, and the Flatpak manifest source tag via `x-release-please-version`/`-version-date` markers; `skip-changelog: true` keeps no `CHANGELOG.md` in-repo). On merge: creates the `v*` tag + GitHub Release with release notes.
- `.github/workflows/release.yml` — manual `workflow_dispatch` calls `_package.yml` to build all 4 formats as artifacts without publishing (test lane); tag push `v*` calls `_package.yml` (also uploading `flathub-bundle`), uploads the 4 formats to the GitHub Release, and publishes `.snap` to `candidate`.
- `me.ariyadey.capturist.metainfo.xml` holds a single `<release>` entry with `<url type="details">` (release history lives on GitHub Releases), updated only by `release-please` and never mutated at build time. Snapcraft adopts metadata from `parse-info` and overrides `version` via `craftctl set version` when `snap-version.txt` exists.
- Frontend unit tests run through the `@angular/build:unit-test` builder (jsdom/Vitest); Tauri APIs are mocked in `src/test-setup.ts` via `@tauri-apps/api/mocks`.

## Critical invariants (read before touching these areas)

- **Deep links (Linux):** a URL arrives in a new process's `argv` and reaches the primary only through single-instance DBus forwarding. Handle `capturist://oauth?...` in `on_another_instance_trial`; `on_open_url` never fires in the running instance.
- **Snap single-instance name:** the DBus name in `snapcraft.yaml` must exactly match the pinned plugin's runtime name (`me.ariyadey.capturist.SingleInstance` for `tauri-plugin-single-instance` v2.4.3).
- **Todoist OAuth:** PKCE public client, no secret. Access tokens expire in 1h; refresh tokens rotate, so never replay one; deduplicate concurrent refreshes.
- **Angular build:** keep `undici`, `fs`, `path` in `angular.json` `externalDependencies`, or the production bundle fails on Node-only SDK imports.
- **Flatpak CI:** `_package.yml` rewrites the `capturist` git source in `me.ariyadey.capturist.yml` to the build commit (`commit:` for dev builds; `tag:` + `commit:` for `v*` releases) via `sed` + `grep` guard; do not depend on the tag existing during dev builds.
