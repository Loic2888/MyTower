# CLAUDE.md

## Contexte du projet

Plateforme locale, 100% offline, distribuée sous forme de services Docker Compose
orchestrés par un shell natif **Tauri** (Rust) affichant une UI **Svelte + Vite**.
Écran divisé en deux : chat IA (assistant local) à droite, outils à gauche (un clic
sur un outil l'ouvre dans le panneau de gauche). Zéro dépendance internet en
fonctionnement normal — chaque service tourne en local, y compris le LLM.

Développement principalement en mode agentique (Claude Code). Avant de coder quoi
que ce soit, vérifier l'étape actuelle dans `TODO.md` et ne pas anticiper les
étapes suivantes (voir "Séquencement" plus bas).

## Stack canonique — ne pas dévier sans décision actée dans docs/decisions/

- **Shell natif** : Tauri (Rust). Pas Electron, pas Wails.
- **Frontend** : Svelte + Vite, TypeScript. Styling en **Tailwind CSS v4**
  via le plugin officiel `@tailwindcss/vite` (pas de config PostCSS séparée).
  Dans `vite.config.ts`, déclarer `tailwindcss()` avant `svelte()` dans la
  liste des plugins — l'ordre compte.
- **Orchestration des services** : Docker Compose, un service = un conteneur.
  Ne jamais faire tourner un service critique hors Docker "pour tester vite" —
  la config doit rester reproductible.
- **LLM local** : servi via Ollama. Modèle par défaut : famille Qwen3.5/3.6,
  `4b` (tag exact `qwen3.5:4b` — voir `docs/decisions/0002-llm-qwen3-4b.md` ;
  benchmark réel sur la machine cible : ~9,4 tokens/s, tool-calling natif
  vérifié fonctionnel), `9b` puis `14b` en repli si la qualité RAG/tool-
  calling l'exige. Alternative de secours : distillations DeepSeek-R1
  (8B/14B). Ne jamais proposer Kimi K3 ou tout modèle dense >30B / MoE >50B
  total — hors de portée de la cible matérielle.
- **Embeddings (RAG)** : `nomic-embed-text` via Ollama.
- **Vector store (RAG)** : Qdrant.
- **Chat P2P** : Reticulum (RNS) + LXMF. Pas Bitchat/Briar/Meshtastic pris
  séparément — un seul protocole doit unifier les transports BLE et LoRa sous
  une identité unique. Nœuds de propagation activés (store-and-forward par
  relais, pas un simple recroisement d'appareils).
- **Académie** : Kolibri (conteneur dédié, contenu type Khan Academy).
- **Contenus hors ligne** : Kiwix (Wikipedia/Wiktionnaire en `.zim`, voir
  `docs/decisions/0003-kiwix-offline-wikipedia.md`), catalogué dans
  `collections/` et téléchargé via l'outil "Contenus".
- **Cartes** : ProtoMaps (tuiles OSM offline).
- **Salle de jeux** : émulateurs (consoles rétro) + jeux programmés pour
  jouer contre une IA locale. Échecs en premier : `python-chess` pour les
  règles/notation, Stockfish comme adversaire IA (protocole UCI standard —
  n'importe quel moteur UCI fonctionne par notation de coups nativement,
  pas besoin d'un moteur "spécial jeu par message"). Voir
  `docs/decisions/0004-retrait-cyberchef-emulateurs.md`.

## Contraintes matérielles

Cible : PC i7, 32 Go RAM, GPU **intégré** (pas de GPU dédié). Inférence LLM en
CPU/iGPU. Ne jamais recommander ou intégrer un modèle qui suppose un GPU dédié
ou un cluster.

## Explicitement écarté — ne pas réintroduire sans repasser par une décision actée

- Apprentissage continu / fine-tuning périodique du LLM sur les échanges :
  écarté au profit du RAG (Qdrant + fenêtre glissante 30 jours).
- Réseau "île-à-île" à distance entre plateformes : abandonné. Le chat
  BLE/LoRa suffit comme communication.
- Bitchat, Briar, Meshtastic comme briques du produit : servent de référence
  de design uniquement, jamais d'intégration directe.
- Identité de découverte "jetable"/changeante : écarté. Une seule identité
  stable par utilisateur ; le statut "contact" s'ajoute après échange
  explicite de QR code, il ne remplace pas l'identité de présence.
- Fork intégral de Project NOMAD (github.com/Crosstalk-Solutions/project-nomad) :
  NOMAD sert de référence d'architecture (choix de briques : Ollama+Qdrant,
  Kolibri, ProtoMaps, Docker Compose), pas de base de code à forker.
- CyberChef : retiré du projet (voir
  `docs/decisions/0004-retrait-cyberchef-emulateurs.md`). Ne pas le
  réintégrer sans repasser par une décision actée.

## Configuration & sécurité

- **Variables d'environnement** : toute valeur de configuration
  (clés/tokens, ports, chemins d'hôte, URLs de service, identifiants de
  modèle...) va dans `.env`, jamais hardcodée dans le code source. `.env`
  est dans `.gitignore` — ne jamais le committer. Fournir un `.env.example`
  à jour (versionné, sans valeur réelle) à chaque nouvelle variable ajoutée,
  pour que la config reste reproductible.
- **Pas de secrets en dur** : aucune clé, token, mot de passe, chemin
  absolu propre à une machine ou credential ne doit apparaître en clair
  dans le code, les fichiers de config versionnés, `tauri.conf.json` ou
  `docker-compose.yml`. Utiliser des variables d'environnement (interpolées
  dans `docker-compose.yml` via `${VAR}`) ou un mécanisme de secret dédié.
- **Sécurité pendant le développement** : garder à l'esprit les failles
  courantes à chaque ajout de code (injection, XSS, désérialisation non
  validée, permissions Tauri trop larges dans `src-tauri/capabilities/`,
  CSP désactivée sans raison, ports exposés sans nécessité dans
  `docker-compose.yml`...). Ne jamais désactiver une vérification de
  sécurité "pour aller plus vite" sans le documenter dans
  `docs/decisions/`. Valider les entrées aux frontières (IPC Tauri,
  requêtes vers les services Docker) plutôt que de faire confiance
  aveuglément aux données reçues.

## Mémoire RAG

Fenêtre glissante de 30 jours. Job planifié de purge des entrées Qdrant plus
anciennes que 30 jours, basé sur un timestamp en métadonnée. Ne pas indexer
indéfiniment.

## Mode vocal

Activation/désactivation strictement par bouton (push-to-talk). Pas d'écoute
continue, pas de détection de mot de réveil pour l'instant. Pipeline : bouton
pressé → whisper.cpp (STT) → même boucle LLM que le chat texte → Piper (TTS,
voix française disponible nativement) → lecture audio.

## Séquencement du développement

Respecter l'ordre ci-dessous. `TODO.md` fait foi pour le détail et l'état
d'avancement à jour.

1. Structure du repo + première version du front (Tauri + Svelte, panneau
   outils vide, panneau chat vide).
2. Chat IA (assistant) connecté au front — brancher sur l'API Ollama. Sert
   aussi de banc de test pour comparer les tailles de modèle.
3. Outils, un par un, **sans tool-calling à ce stade** : Kolibri → Contenus
   (Kiwix) → GPS/ProtoMaps → salle de jeux (émulateurs + échecs) → chat P2P
   (le plus complexe, en dernier de cette phase).
4. Mode vocal.
5. Tool-calling, outil par outil (commencer par les échecs — actions
   déterministes, faciles à vérifier), en tout dernier.

Ne jamais brancher le tool-calling sur un outil avant que cet outil ne
fonctionne de façon autonome. Ne jamais paralléliser plusieurs intégrations
outil↔IA en une seule étape — une seule à la fois, valider, puis la suivante.

## Format des manifestes d'outils (pour le tool-calling, phase 5)

Chaque outil devra exposer ses actions en JSON Schema (nom, description,
paramètres) pour que l'IA puisse les appeler de façon uniforme. Concevoir
chaque service outil avec ce contrat en tête dès sa création, même si le
branchement à l'IA vient en dernier — ça évite de devoir réécrire l'API de
chaque outil plus tard.

## Organisation des fichiers projet

- `CLAUDE.md` (ce fichier) : reste court, pointe vers le reste, ne pas y
  dupliquer ce qui est dérivable du code.
- `.claude/rules/` : un fichier par sous-système (`chat-p2p.md`,
  `docker-services.md`, `ia-rag.md`, `tool-manifests.md`...), créés au fur et
  à mesure. Chargés automatiquement par Claude Code, pas besoin d'import.
- `docs/decisions/` : une décision d'architecture = un fichier
  (`0001-shell-tauri.md`, `0002-llm-qwen3.md`...). Ajouter une entrée à chaque
  choix structurant, ne jamais réécrire l'historique déjà consigné.
- `TODO.md` : roadmap vivante, à jour de l'étape en cours.
