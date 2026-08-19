use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Window};

use crate::http::http_client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChatChunk {
    message: Option<OllamaChatChunkMessage>,
    #[serde(default)]
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaChatChunkMessage {
    content: String,
}

fn ollama_base_url() -> String {
    std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string())
}

fn ollama_model() -> String {
    std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen3.5:4b".to_string())
}

/// Poussé en tête de chaque conversation. Deux objectifs à la fois : un ton
/// chaleureux et attentionné (demandé pour que l'assistant soit accueillant
/// et s'intéresse à ce que fait l'utilisateur), et des réponses courtes —
/// sur cette machine (CPU/iGPU), le temps de réponse est dominé par le
/// nombre de tokens générés, donc la brièveté aide directement le budget de
/// latence du projet.
const SYSTEM_PROMPT: &str = "Tu es un assistant local, chaleureux et attentionné. \
Sois amical et à l'écoute, montre un intérêt sincère pour ce que fait \
l'utilisateur. Réponds toujours de façon brève et directe : quelques \
phrases maximum, sans détailler ni faire de liste inutilement. Développe \
uniquement si l'utilisateur le demande explicitement.";

/// Envoie une liste de messages à Ollama et pousse chaque morceau de
/// réponse au front via l'événement `chat:chunk`. Le résultat (résolu/
/// rejeté) sert de signal de fin/erreur — pas besoin d'événement dédié.
/// Partagé entre `send_chat_message` (message de l'utilisateur) et
/// `trigger_checkin` (message déclenché par l'app elle-même).
async fn stream_chat(
    window: &Window,
    messages: Vec<serde_json::Value>,
    think: bool,
) -> Result<(), String> {
    let base_url = ollama_base_url();
    let url = format!("{base_url}/api/chat");

    let body = serde_json::json!({
        "model": ollama_model(),
        "messages": messages,
        "stream": true,
        "think": think,
        // Garde le modèle chargé en mémoire 10 min après chaque échange —
        // évite de repayer le rechargement (~15-20s observés) si les
        // messages ne s'enchaînent pas immédiatement.
        "keep_alive": "10m",
    });

    let response = http_client()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Impossible de joindre Ollama ({base_url}) : {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Ollama a répondu avec le statut {}", response.status()));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| format!("Erreur de flux Ollama : {e}"))?;
        buffer.push_str(&String::from_utf8_lossy(&bytes));

        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer.drain(..=newline_pos);
            if line.is_empty() {
                continue;
            }

            let parsed: OllamaChatChunk = serde_json::from_str(&line)
                .map_err(|e| format!("Réponse Ollama illisible : {e}"))?;

            if let Some(message) = parsed.message {
                if !message.content.is_empty() {
                    window
                        .emit("chat:chunk", message.content)
                        .map_err(|e| format!("Erreur d'émission d'événement : {e}"))?;
                }
            }

            if parsed.done {
                return Ok(());
            }
        }
    }

    Ok(())
}

/// `think` est piloté depuis le front (bouton "Réflexion") : `qwen3.5:9b`
/// reste silencieux ~40s en pensant, donc désactivé par défaut côté UI,
/// mais activable au cas par cas.
#[tauri::command]
pub async fn send_chat_message(
    window: Window,
    history: Vec<ChatMessage>,
    think: bool,
) -> Result<(), String> {
    let mut messages = Vec::with_capacity(history.len() + 1);
    messages.push(serde_json::json!({ "role": "system", "content": SYSTEM_PROMPT }));
    messages.extend(
        history
            .into_iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content })),
    );

    stream_chat(&window, messages, think).await
}

/// Déclenché par l'app (pas par l'utilisateur) quand un outil se ferme —
/// demande à l'assistant de prendre des nouvelles, dans le même style que
/// les réponses normales. Réflexion toujours désactivée : c'est une
/// relance courte, pas une question qui demande d'y réfléchir.
#[tauri::command]
pub async fn trigger_checkin(window: Window, tool_label: String) -> Result<(), String> {
    let nudge = format!(
        "L'utilisateur vient de quitter l'outil « {tool_label} ». Demande-lui \
avec attention, en une phrase courte et chaleureuse, comment ça s'est passé."
    );

    let messages = vec![
        serde_json::json!({ "role": "system", "content": SYSTEM_PROMPT }),
        serde_json::json!({ "role": "system", "content": nudge }),
    ];

    stream_chat(&window, messages, false).await
}
