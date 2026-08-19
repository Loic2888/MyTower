# 0001 — Démarrer directement sur Qwen3.5/3.6 8b (pas 4b)

Date : 2026-08-14

> **Remplacée par [0002-llm-qwen3-4b.md](0002-llm-qwen3-4b.md)** (même jour) :
> le `9b` est finalement redescendu en `4b` après benchmark réel sur la
> machine cible. Ce fichier reste tel quel pour l'historique.

## Contexte

`CLAUDE.md` (Stack canonique) prévoyait de commencer l'itération sur le
modèle `4b` et de ne monter en taille (`8b`/`14b`) que si la qualité
RAG/tool-calling l'exigeait, pour itérer vite sur la cible matérielle
(i7, 32 Go RAM, GPU intégré, inférence CPU/iGPU).

## Décision

On démarre directement en `8b`. Le `4b` est jugé trop limité pour être
un point de départ utile — passer par lui aurait juste ajouté un aller-retour
inutile.

**Tag Ollama exact retenu : `qwen3.5:9b`.** Vérification faite sur la
bibliothèque Ollama (2026-08-14) : `qwen3.6:8b` n'existe pas — la famille
Qwen3.6 saute directement de `27b` à `35b`+ (17-24 Go), bien au-delà de la
cible matérielle (32 Go RAM, iGPU). Qwen3.5 a en revanche un tag `9b`, le
plus proche de l'intention initiale et compatible avec la contrainte
matérielle. C'est ce tag qui est utilisé comme `OLLAMA_MODEL` par défaut
(voir `.env.example`).

**Réflexion ("thinking") désactivée par défaut, pilotable depuis l'UI.**
`qwen3.5:9b` est un modèle "thinking" : testé en direct, la phase de
réflexion seule prend ~40s pour une réponse triviale — bien au-delà du
budget de 20s max jugé acceptable pour ce projet. Un bouton dans
`ChatPanel.svelte` ("Réflexion activée/désactivée") pilote le paramètre
`think` envoyé à Ollama à chaque requête (`src-tauri/src/ollama.rs`,
commande `send_chat_message`) — désactivé par défaut, activable au cas par
cas. Réflexion désactivée, le temps de première réponse tombe à ~1s. Le
temps de génération pur (hors réflexion) reste par contre modeste sur cette
machine : ~5,5 tokens/s observés en CPU/iGPU, donc une réponse de quelques
phrases peut quand même approcher les 20s.

**Prompt système pour des réponses courtes.** Un message système fixe
(`SYSTEM_PROMPT` dans `src-tauri/src/ollama.rs`) est ajouté en tête de
chaque conversation pour demander des réponses brèves par défaut — sur
cette machine, le temps de réponse est dominé par le nombre de tokens
générés, donc raccourcir les réponses aide directement le budget de
latence.

## Conséquences

- Étape 2 du séquencement (chat IA connecté au front) utilise `8b` comme
  modèle par défaut dès le premier branchement à l'API Ollama.
- Le `14b` reste l'option de repli si la qualité RAG/tool-calling l'exige
  plus tard (inchangé par rapport à `CLAUDE.md`).
- À surveiller : le `8b` est plus lourd que le `4b` sur CPU/iGPU pur — si les
  temps de réponse deviennent gênants pendant les tests de l'étape 2,
  redescendre en `4b` reste une option, pas un aveu d'échec.
