// Kalimat desktop — a native window pointed at the real, deployed web app
// (kalimat.site). No local frontend bundle: the site already got a proper
// desktop layout (sidebar, wide grids) this session, so this shell just
// gives it a dock/taskbar icon, its own window, and no browser chrome —
// same shape as Slack/Discord's first desktop clients before they went
// deeper into native territory. No local business logic to keep in sync.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Google/Apple OAuth can't run inside this embedded webview (both
    // reject sign-in from an embedded/native user agent) — the web app
    // opens the consent screen in the system browser instead
    // (lib/tauriAuth.ts, mirrors the mobile app's identical fix) and
    // Google/Supabase redirect back to kalimat://auth-callback, which the
    // OS hands to this app via the deep-link plugin below. The scheme
    // itself is registered at BUNDLE time from tauri.conf.json's
    // plugins.deep-link config (baked into Info.plist on macOS, the
    // installer registry on Windows — confirmed present in the built
    // Info.plist via `plutil -p`) — no runtime `.register()` call needed
    // for a bundled build, and calling it anyway crashed the app on
    // macOS at launch (did_finish_launching aborted) since it's meant for
    // unbundled `cargo run` iteration on platforms without a manifest to
    // read, not a `cargo tauri build` bundle.
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .run(tauri::generate_context!())
        .expect("error while running kalimat desktop");
}
