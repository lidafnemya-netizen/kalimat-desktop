# Kalimat Desktop

Native Mac + Windows shell around the real, deployed web app
(`https://kalimat.site`) — a window with no browser chrome, a dock/taskbar
icon, nothing else. No local business logic: every feature lives on
kalimat.site itself (which got a real desktop layout — sidebar, wide grids —
in the same session this wrapper was built), so this app updates itself
automatically the moment the site deploys. No app-store review needed for
content changes, only for the shell itself.

Built with [Tauri v2](https://tauri.app) — a native window (WebView2 on
Windows / WKWebView on Mac, both already on the OS, no bundled Chromium) —
instead of Electron, so the installer is a few MB instead of ~150MB.

## Status (2026-08-20)

- ✅ Builds and runs locally (`Kalimat.app`, verified launches and stays
  running — could not screenshot the window itself, this dev environment has
  no attached display, but the process runs clean with no crash).
- ⚠️ `.dmg` bundling failed in this sandboxed dev environment specifically
  (`bundle_dmg.sh` / `hdiutil` needs disk-image permissions this sandbox
  doesn't grant) — the `.app` itself built fine. **Rebuild locally in a
  normal Terminal** (not sandboxed) to get the real `.dmg` — should just
  work; this is an environment quirk, not a config bug.
- ❌ **Not code-signed / not notarized.** Needs the paid Apple Developer
  Program ($99/yr, not purchased yet — same account referenced elsewhere in
  the Kalimat project for iOS TestFlight). Without it:
  - Anyone opening the unsigned `.app` on their own Mac gets a Gatekeeper
    warning and must right-click → Open once to bypass it. Not suitable for
    the general public via the vitrine site as-is.
  - Works fine for your own testing right now.
- ❌ **Windows build not yet produced** — this is a Mac dev machine, and
  Tauri cross-compiles the *Rust binary* but not the platform-specific
  bundler (NSIS/MSI needs to run ON Windows, or via CI). See below.

## Build locally (Mac)

```bash
cd kalimat-desktop
source "$HOME/.cargo/env"   # if cargo isn't already on PATH
cargo tauri build            # release build + .app + .dmg
```

Debug build (faster, for quick checks): `cargo tauri build --debug`.

## Getting a real Windows build without owning a Windows machine

Set up `.github/workflows/build.yml` (not added yet — do this next) using
`tauri-apps/tauri-action`, matrix `[macos-latest, windows-latest]`. Push to
GitHub, the Windows runner produces a real signed-or-unsigned `.msi`/`.exe`
as a build artifact — no local Windows install needed. Mac signing still
needs the Apple Developer cert either way; Windows unsigned `.exe` triggers
a SmartScreen warning but is otherwise fully installable/runnable (much
less friction than macOS Gatekeeper — Windows doesn't hard-block it).

## Before putting this on kalimat-vitrine

1. Apple Developer Program → code-sign + notarize the Mac build (removes
   the Gatekeeper warning entirely).
2. CI workflow (above) → real Windows `.exe`/`.msi`.
3. Auto-update: Tauri has a built-in updater plugin — worth wiring up later
   so shell updates (icon, window behavior) don't require everyone to
   re-download; page content always auto-updates already since it's just
   the live website.
