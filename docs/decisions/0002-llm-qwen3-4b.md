# 0002 — Redescendre sur Qwen3.5 4b (au lieu de 9b)

Date : 2026-08-14

## Contexte

[0001-llm-qwen3-8b.md](0001-llm-qwen3-8b.md) avait choisi `qwen3.5:9b` comme
modèle par défaut. Testé en conditions réelles sur la machine cible (i7,
32 Go RAM, iGPU, `think: false`), il tient le budget de latence de 20s
max mais avec peu de marge : ~5,5 tokens/s, soit ~20s pour une réponse
d'une centaine de tokens — pile à la limite.

En parallèle, une recherche sur les modèles compacts adaptés à la
conversation, au RAG et aux décisions de tool-calling a confirmé que des
modèles sous les 5B (Qwen3.5 4B, Llama 3.2 3B, Phi-4-mini...) tiennent
correctement ces trois usages sur CPU. `qwen3.5:4b` a été pullé et
benchmarké en direct sur cette machine pour vérifier avant de trancher.

## Résultats du benchmark (`qwen3.5:4b`, modèle chargé en mémoire)

- **Vitesse** : ~9,4 tokens/s (vs ~5,5 pour le `9b`) — une réponse de
  ~100 tokens tombe à ~12s au lieu de ~20s. Marge confortable sur le
  budget de 20s.
- **Tool-calling** : testé avec un outil factice (`get_weather`) — le
  modèle choisit correctement l'outil et extrait l'argument attendu
  (`{"city": "Paris"}`). Capacité `tools` confirmée native (`/api/show`).
- **Conversation** : réponses cohérentes, mais une hallucination factuelle
  observée sur une question technique ouverte (détail inventé sur
  l'architecture de Tauri). Comportement attendu à cette taille — le RAG
  (étape 2/3, mémoire Qdrant) doit limiter ce risque en ancrant les
  réponses sur des documents récupérés plutôt que sur la mémoire du
  modèle seule.

## Décision

`OLLAMA_MODEL` par défaut passe de `qwen3.5:9b` à **`qwen3.5:4b`**.

## Conséquences

- `.env.example`, `CLAUDE.md` (Stack canonique) mis à jour.
- Le `9b` reste une option de repli si la qualité s'avère insuffisante à
  l'usage (ex. tool-calling peu fiable en conditions réelles avec des
  outils plus complexes que le test effectué ici) ; le `14b` reste
  l'option suivante, dans cet ordre.
- Ne pas conclure trop vite sur la fiabilité factuelle du `4b` à partir
  d'un seul test — à réévaluer une fois le RAG branché (étape 2/3), qui
  change la donne en ancrant les réponses sur des sources.
