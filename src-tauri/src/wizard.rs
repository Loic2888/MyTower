use std::path::PathBuf;

use futures_util::StreamExt;
use serde::Serialize;
use tauri::{Emitter, Window};
use tokio::io::AsyncWriteExt;

use crate::http::http_client;
use crate::paths::project_root;

fn kiwix_data_dir() -> PathBuf {
    // Doit rester en phase avec le bind-mount `./data/kiwix` de
    // docker-compose.yml (résolu, lui, depuis la racine du repo) — un
    // chemin relatif dans `.env` (`KIWIX_DATA_DIR=./data/kiwix`, la valeur
    // par défaut de `.env.example`) est donc résolu depuis la racine du
    // projet, pas depuis le dossier courant au runtime (`src-tauri/`).
    let dir = std::env::var("KIWIX_DATA_DIR").unwrap_or_else(|_| "./data/kiwix".to_string());
    let path = PathBuf::from(dir);
    if path.is_absolute() {
        path
    } else {
        project_root().join(path)
    }
}

#[derive(Serialize, Clone)]
struct DownloadProgress {
    id: String,
    downloaded: u64,
    total: Option<u64>,
}

/// Télécharge un fichier .zim (collections/wikipedia.json) vers le dossier
/// de données Kiwix, en streaming pour ne pas charger tout le fichier en
/// mémoire (certains .zim font plusieurs Go). Pousse la progression au
/// front via l'événement `wizard:progress`, dans le même esprit que
/// `chat:chunk` côté ollama.rs. Un seul téléchargement à la fois est géré
/// côté front (boutons désactivés pendant l'opération) ; pas d'annulation
/// possible dans cette première version.
#[tauri::command]
pub async fn download_zim(
    window: Window,
    id: String,
    url: String,
    filename: String,
) -> Result<(), String> {
    let dir = kiwix_data_dir();
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Impossible de créer {dir:?} : {e}"))?;

    let response = http_client()
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Impossible de joindre {url} : {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Téléchargement échoué avec le statut {}", response.status()));
    }

    let total = response.content_length();
    let path = dir.join(&filename);
    // Fichier temporaire pendant le téléchargement — évite qu'un .zim
    // partiel/corrompu soit pris pour du contenu valide par kiwix-serve
    // si le téléchargement est interrompu en cours de route.
    let tmp_path = dir.join(format!("{filename}.part"));

    let mut file = tokio::fs::File::create(&tmp_path)
        .await
        .map_err(|e| format!("Impossible de créer {tmp_path:?} : {e}"))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| format!("Erreur de flux : {e}"))?;
        file.write_all(&bytes)
            .await
            .map_err(|e| format!("Erreur d'écriture sur disque : {e}"))?;

        downloaded += bytes.len() as u64;
        window
            .emit(
                "wizard:progress",
                DownloadProgress {
                    id: id.clone(),
                    downloaded,
                    total,
                },
            )
            .map_err(|e| format!("Erreur d'émission d'événement : {e}"))?;
    }

    tokio::fs::rename(&tmp_path, &path)
        .await
        .map_err(|e| format!("Impossible de finaliser {path:?} : {e}"))?;

    Ok(())
}
