use std::sync::OnceLock;

/// Un seul client HTTP réutilisé pour tous les appels (Ollama, téléchargements
/// du wizard...) — en créer un neuf à chaque fois referait le travail de
/// connexion pour rien.
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}
