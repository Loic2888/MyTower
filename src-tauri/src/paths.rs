use std::path::PathBuf;

/// Racine du repo (parent de `src-tauri/`), calculée à la compilation via
/// `CARGO_MANIFEST_DIR` plutôt que déduite du dossier courant au runtime.
/// Important : `cargo tauri dev` lance le binaire avec `src-tauri/` comme
/// dossier courant, pas la racine du projet — un chemin relatif du genre
/// `"./data/kiwix"` résolu au runtime pointerait donc au mauvais endroit
/// (`src-tauri/data/kiwix` au lieu de `data/kiwix` à la racine, là où
/// `docker-compose.yml` monte réellement le dossier).
pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri devrait toujours avoir un dossier parent")
        .to_path_buf()
}
