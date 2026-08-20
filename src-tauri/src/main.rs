// Kalimat desktop — a native window pointed at the real, deployed web app
// (kalimat.site). No local frontend bundle: the site already got a proper
// desktop layout (sidebar, wide grids) this session, so this shell just
// gives it a dock/taskbar icon, its own window, and no browser chrome —
// same shape as Slack/Discord's first desktop clients before they went
// deeper into native territory. No local business logic to keep in sync.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Fixed port for the local OAuth callback listener — must match the
// redirectTo built in lib/tauriAuth.ts and the http://localhost:17683/**
// entry in Supabase's redirect URL allow list.
const OAUTH_CALLBACK_PORT: u16 = 17683;

fn main() {
    // Google/Apple OAuth can't run inside this embedded webview (both
    // reject sign-in from an embedded/native user agent) — the web app
    // opens the consent screen in the system browser instead
    // (lib/tauriAuth.ts, mirrors the mobile app's identical fix).
    //
    // The callback used to come back via a kalimat:// deep link, but that
    // proved unreliable on macOS: Safari would sit on a spinner after the
    // Google→Supabase redirect chain and never hand off to the app —
    // confirmed the scheme registration and Rust-side RunEvent::Opened
    // handling both work in isolation (a manual `open kalimat://...` was
    // received correctly), so the failure is specifically in Safari's
    // silent-handoff behavior for a scheme reached via a server-side
    // redirect chain rather than a direct user-initiated navigation —
    // a known rough edge, not something fixable from this app's side.
    //
    // Standard fix (same one VS Code/Slack/Discord use): redirect to a
    // plain http://localhost port instead. Browsers complete a redirect
    // chain into http:// with no special handoff step, no permission
    // dialog, no lost-user-gesture restriction. This tiny server accepts
    // exactly one request, hands the query string to the webview via a
    // Tauri event, and shows a "you can close this tab" page.
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || run_oauth_callback_server(handle));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running kalimat desktop");
}

fn run_oauth_callback_server(app: tauri::AppHandle) {
    use tauri::Emitter;

    let server = match tiny_http::Server::http(format!("127.0.0.1:{OAUTH_CALLBACK_PORT}")) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("oauth callback server failed to bind: {e}");
            return;
        }
    };

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let _ = app.emit("oauth-callback", &url);

        let html = "<!doctype html><html><body style=\"font-family:-apple-system,sans-serif;text-align:center;padding-top:4rem;color:#3A3226\"><p>Connexion réussie — tu peux fermer cet onglet et revenir sur Kalimat.</p></body></html>";
        let response = tiny_http::Response::from_string(html)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
        let _ = request.respond(response);
    }
}
