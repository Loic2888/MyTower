use std::fs::OpenOptions;
use std::io::Write;

use serde::Serialize;
use tauri::{AppHandle, Manager};

#[derive(Serialize)]
struct SessionEvent {
    timestamp: String,
    kind: String,
    label: String,
}

/// Mémoire de session légère : journal horodaté (`session.jsonl`, dans le
/// dossier de données de l'app) des événements d'usage — pour l'instant
/// seulement ouverture/fermeture d'un outil. Base posée à la demande de
/// l'utilisateur en anticipant l'étape 3 (aucun outil n'est encore
/// fonctionnel, donc rien de plus riche à logger pour le moment) : sert de
/// point de départ pour qu'une future version de l'assistant puisse
/// s'appuyer sur "tu as fait X à telle heure" plutôt que de tout redécouvrir
/// à chaque fois.
#[tauri::command]
pub fn log_session_event(app: AppHandle, kind: String, label: String) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Répertoire de données introuvable : {e}"))?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("Impossible de créer {dir:?} : {e}"))?;

    let event = SessionEvent {
        timestamp: chrono::Local::now().to_rfc3339(),
        kind,
        label,
    };
    let line = serde_json::to_string(&event).map_err(|e| format!("Sérialisation : {e}"))?;

    let path = dir.join("session.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("Impossible d'ouvrir {path:?} : {e}"))?;
    writeln!(file, "{line}").map_err(|e| format!("Écriture échouée : {e}"))?;

    Ok(())
}
