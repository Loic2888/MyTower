mod http;
mod ollama;
mod paths;
mod session;
mod wizard;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // `.env` vit à la racine du repo, pas dans src-tauri/ (cwd au runtime) —
  // voir paths::project_root().
  dotenvy::from_path(paths::project_root().join(".env")).ok();

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      ollama::send_chat_message,
      ollama::trigger_checkin,
      session::log_session_event,
      wizard::download_zim,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
