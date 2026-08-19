# TODO — roadmap MyTower

Source de vérité pour l'étape en cours. Voir "Séquencement du développement"
dans `CLAUDE.md` pour le détail des règles (une seule intégration à la fois,
ne jamais anticiper une étape suivante).

## 1. Structure du repo + première version du front — EN COURS

- [x] Scaffold Tauri (Rust) + Svelte + Vite + TypeScript.
- [x] Tailwind CSS v4 via `@tailwindcss/vite` (plugin déclaré avant `svelte()`
      dans `vite.config.ts`).
- [x] Écran divisé : `ToolsPanel.svelte` (gauche, vide) / `ChatPanel.svelte`
      (droite, vide).
- [x] `docker-compose.yml` en squelette (ollama, qdrant, kolibri, protomaps —
      aucun service câblé à l'app pour l'instant).
- [x] Installer les dépendances système Linux de Tauri
      (`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `librsvg2-dev`,
      `libayatana-appindicator3-dev`, `libdbus-1-dev`, `pkg-config`,
      `patchelf`, `build-essential`, `libssl-dev`). `cargo check` passe.
- [ ] Vérifier `cargo tauri dev` avec un environnement graphique disponible
      (le WSL de dev n'a pas d'affichage).

## 2. Chat IA connecté au front — EN COURS

- [x] Modèle `qwen3.5:4b` retenu par défaut (voir
      `docs/decisions/0002-llm-qwen3-4b.md` — `qwen3.5:9b` d'abord pullé et
      testé, puis redescendu en `4b` après benchmark réel : ~9,4 tok/s,
      tool-calling natif vérifié fonctionnel sur un outil de test).
- [x] Backend Rust (`src-tauri/src/ollama.rs`) : commande
      `send_chat_message`, appelle `/api/chat` en streaming, pousse chaque
      morceau de réponse via l'événement `chat:chunk`. Config
      (`OLLAMA_BASE_URL`, `OLLAMA_MODEL`) chargée depuis `.env`.
- [x] `ChatPanel.svelte` câblé : envoi de message, affichage du flux de
      réponse, gestion d'erreur basique.
- [x] Vérifié en direct (curl sur `/api/chat`) : `qwen3.5:9b` est un modèle
      "thinking" — ~40s de silence observés pour une réponse triviale avec
      réflexion activée. Trop lent (budget max 20s).
- [x] Bouton "Réflexion activée/désactivée" dans `ChatPanel.svelte` —
      désactivée par défaut, pilote le paramètre `think` envoyé à Ollama.
- [x] Prompt système ajouté (`SYSTEM_PROMPT` dans `ollama.rs`) pour des
      réponses courtes par défaut, afin d'optimiser le temps de réponse.
- [x] `qwen3.5:4b` : ~9,4 tokens/s en conditions réelles (modèle chargé),
      confortablement sous le budget de 20s. Tool-calling testé avec un
      outil factice (`get_weather`) : choix d'outil et extraction
      d'argument corrects. Une hallucination factuelle observée sur une
      question ouverte — à surveiller, le RAG (étape suivante) devrait
      réduire ce risque en ancrant les réponses sur des sources.
- [ ] Test UI de bout en bout (`cargo tauri dev`) — pas possible depuis ce
      WSL sans affichage graphique, à faire depuis un poste avec GUI.
- [x] Prompt système rendu plus chaleureux/attentionné (toujours en une
      seule constante `SYSTEM_PROMPT`, pas de config séparée).
- [x] Accueil au lancement : `ChatPanel` affiche un message de bienvenue
      statique (tiré au sort parmi quelques variantes, `chatStore.svelte.ts`)
      — pas d'appel Ollama pour ça, pour ne rien ralentir au démarrage.
- [x] **Anticipation consciente de l'étape 3**, demandée explicitement par
      l'utilisateur malgré la règle "ne jamais anticiper" : mémoire de
      session immédiate (`src-tauri/src/session.rs`, commande
      `log_session_event`, écrit dans `session.jsonl` côté données de
      l'app) + check-in automatique de l'assistant (`trigger_checkin`) quand
      un outil se ferme (déclenché depuis `ToolsPanel.svelte`, avec un seuil
      de 4s d'ouverture minimum pour éviter de solliciter Ollama sur un
      simple clic curieux). **Limite actuelle : aucun outil n'étant encore
      fonctionnel, le check-in porte sur une session vide** — le mécanisme
      est posé mais n'aura un sens réel qu'une fois un outil de l'étape 3
      effectivement utilisable (ex. une vraie partie d'échecs terminée).

## 3. Outils (un par un, sans tool-calling)

Ordre initial de `CLAUDE.md` : CyberChef → Kolibri → GPS → Salle de jeux →
Messagerie. **Réordonné**, puis **CyberChef retiré du projet** (voir
`docs/decisions/0004-retrait-cyberchef-emulateurs.md`) : Kolibri est passé
en premier, CyberChef n'a jamais été commencé et ne le sera pas. Ordre
actuel : Kolibri (fait) → Contenus/Kiwix (fait, anticipé) → GPS → Salle de
jeux (émulateurs + échecs) → Messagerie.

- [x] **Kolibri (Académie)** — `docker-compose.yml` corrigé après recherche
      (le squelette initial était faux : volume `/kolibri`, pas
      `/kolibrihome`, le conteneur refuse sinon de démarrer ; port 8081
      obligatoire en plus du 8080, sert le contenu zip). **Testé** :
      `docker compose up -d kolibri` démarre bien le conteneur
      (`learningequality/kolibri:latest`), répond en HTTP 302 sur
      `http://localhost:8080` (redirection normale vers l'assistant de
      configuration au premier lancement).
      **Correction en cours de route** : Kolibri envoie
      `X-Frame-Options: DENY`, l'affichage en iframe dans `ToolsPanel` était
      donc impossible (page blanche, pas un problème de latence). Changé
      pour une ouverture dans le **navigateur système** via
      `@tauri-apps/plugin-opener` (`openUrl`) — plus léger que d'ouvrir une
      2ᵉ fenêtre Tauri (même moteur WebKitGTK déjà identifié comme coûteux
      dans cet environnement). Conséquence : plus de suivi ouverture/
      fermeture ni de check-in pour Kolibri spécifiquement (impossible de
      savoir quand un onglet de navigateur externe se ferme).
- [ ] **`protomaps/pmtiles-server` (squelette `docker-compose.yml`,
      service GPS) n'existe pas** — `docker compose up` sans argument échoue
      dessus (`pull access denied`). Non bloquant pour l'instant (`kolibri`
      démarré individuellement), mais à corriger avant l'étape GPS/ProtoMaps
      — même genre de vérification que pour Kolibri à refaire.
- [x] **Dossier `collections/` + outil "Contenus" façon Project NOMAD**
      (référence d'architecture citée dans `CLAUDE.md`), demandé
      explicitement par l'utilisateur après avoir remarqué que Kolibri a
      besoin d'internet pour être rempli de contenu. **Anticipe l'étape 3**
      (ajoute Kiwix, un outil non prévu dans `CLAUDE.md` — voir
      `docs/decisions/0003-kiwix-offline-wikipedia.md`) — décision
      explicite de l'utilisateur, scope volontairement limité (cartes
      reportées à l'étape GPS dédiée, pas de gestion de canaux Kolibri
      dans le wizard — renvoie vers l'import intégré de Kolibri lui-même).
      - `collections/wikipedia.json` : 4 fichiers `.zim` réels, tailles
        vérifiées sur `lb.download.kiwix.org` (4,4 Mo à 4,6 Go).
      - `src-tauri/src/wizard.rs` (commande `download_zim`) : téléchargement
        streamé vers `data/kiwix/` (bind-mount, pas un volume Docker nommé,
        pour que le backend Rust puisse y écrire), progression poussée via
        l'événement `wizard:progress`.
      - `ToolsPanel.svelte` : nouvel outil "Contenus" (6ᵉ icône), liste les
        3 catégories, boutons de téléchargement réels pour Wikipedia/Kiwix.
      - **Testé de bout en bout** : `docker compose up -d kiwix`,
        téléchargement de `wikipedia_en_100_mini` (4,4 Mo, taille conforme),
        `docker compose restart kiwix`, contenu servi (HTTP 200 sur
        `localhost:8082`). Trois bugs trouvés et corrigés au passage — voir
        `docs/decisions/0003-kiwix-offline-wikipedia.md` : (1) dossier
        courant du process ≠ racine du repo au runtime (`.env`/
        `KIWIX_DATA_DIR` ne se seraient pas résolus au bon endroit, corrigé
        via `src-tauri/src/paths.rs`), (2) `kiwix-serve` refuse de démarrer
        sans `.zim` déjà présent, (3) ne détecte pas un `.zim` ajouté à
        chaud — besoin d'un redémarrage du conteneur.
- [ ] GPS/ProtoMaps
- [ ] Salle de jeux — émulateurs (consoles rétro, choix technique pas
      encore tranché) + jeux programmés contre IA locale. Échecs en premier
      (python-chess + Stockfish, UCI) — pourra alimenter
      `log_session_event`/`trigger_checkin` avec de vrais événements de
      partie une fois fonctionnelle.
- [ ] Chat P2P / Messagerie par ondes radio (LoRa) et Bluetooth (BLE) via
      Reticulum/LXMF — le plus complexe, en dernier.

## 4. Mode vocal

- [ ] Pipeline bouton pressé → whisper.cpp (STT) → boucle LLM → Piper (TTS,
      voix française) → lecture audio. Push-to-talk uniquement, pas d'écoute
      continue.

## 5. Tool-calling (en tout dernier, outil par outil)

- [ ] Commencer par les échecs (actions déterministes, faciles à vérifier).
- [ ] Ne brancher le tool-calling sur un outil qu'une fois cet outil
      fonctionnel de façon autonome.
