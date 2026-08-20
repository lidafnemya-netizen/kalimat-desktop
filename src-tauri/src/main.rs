// Kalimat desktop — a native window pointed at the real, deployed web app
// (kalimat.site). No local frontend bundle: the site already got a proper
// desktop layout (sidebar, wide grids) this session, so this shell just
// gives it a dock/taskbar icon, its own window, and no browser chrome —
// same shape as Slack/Discord's first desktop clients before they went
// deeper into native territory. No local business logic to keep in sync.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running kalimat desktop");
}
