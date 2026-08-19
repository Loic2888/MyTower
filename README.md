# MyTower

> ⏸️ **Projet en pause, pour une durée indéterminée.** Je me concentre
> actuellement sur l'apprentissage du métier de *Revenue Engineering*. Le
> code reste tel quel en l'état — voir [`TODO.md`](TODO.md) pour
> l'avancement au moment de la pause.

**Une tour personnelle, autonome et hors ligne.** MyTower est une
plateforme locale qui réunit un assistant IA, une académie, une
bibliothèque de connaissances, une messagerie de proximité et une salle de
jeux — le tout dans une seule application de bureau, et sans jamais dépendre
d'internet pour fonctionner au quotidien.

## Pourquoi hors ligne ?

La plupart des outils numériques du quotidien — assistant IA, cours en
ligne, encyclopédie, messagerie — supposent une connexion internet
permanente et un service tiers quelque part dans un cloud. MyTower part du
principe inverse : **tout tourne en local, sur ta propre machine**, et
internet devient un luxe ponctuel plutôt qu'une dépendance de base.

Ça change concrètement plusieurs choses :

- **Ça continue de marcher quand le réseau ne marche plus.** Coupure
  internet, zone blanche, panne du fournisseur d'accès, tempête, voyage en
  zone isolée, bateau, refuge en montagne : le chat IA, les cours, la carte
  et l'encyclopédie restent utilisables exactement comme d'habitude.
- **Aucune donnée ne sort de la machine.** Les conversations avec
  l'assistant, l'historique d'apprentissage, les messages échangés — rien
  n'est envoyé à un service tiers, il n'y a pas de télémétrie à couper ni de
  politique de confidentialité à surveiller. C'est structurellement privé,
  pas juste par promesse.
- **Rien ne peut disparaître du jour au lendemain.** Un service cloud peut
  fermer, changer de modèle économique ou couper l'accès gratuit. Une
  application qui tourne en local sur du logiciel libre ne dépend de la
  survie commerciale de personne.
- **Pas d'abonnement, pas de coût récurrent.** Le LLM, l'hébergement des
  cours, la carte et l'encyclopédie tournent sur le matériel qu'on possède
  déjà — pas de facture d'API IA qui grimpe avec l'usage.
- **Utile en contexte dégradé ou d'urgence.** C'est directement l'esprit de
  [Project NOMAD](https://github.com/Crosstalk-Solutions/project-nomad),
  qui sert de référence d'architecture à MyTower : avoir accès au savoir et
  à la communication même quand l'infrastructure normale est coupée
  (catastrophe naturelle, zone reculée, coupure prolongée).
- **Adapté à un usage éducatif encadré.** Une académie et une encyclopédie
  hors ligne, c'est un enfant qui peut apprendre et chercher sans avoir
  besoin d'un accès internet non supervisé.

Le seul moment où internet est utile, c'est pour **remplir** la
plateforme au départ (télécharger les cours, l'encyclopédie, les mises à
jour) — jamais pour l'utiliser au quotidien.

## Fonctionnalités

- **Chat IA local** — assistant conversationnel propulsé par un LLM
  (famille Qwen) servi via [Ollama](https://ollama.com), entièrement en
  CPU/GPU intégré, sans service cloud.
- **Académie** — cours façon Khan Academy via
  [Kolibri](https://learningequality.org/kolibri/), la plateforme
  éducative hors ligne de Learning Equality.
- **Contenus hors ligne** — Wikipedia et Wiktionnaire consultables sans
  connexion via [Kiwix](https://www.kiwix.org/), avec un catalogue de
  contenus prêts à télécharger (dossier `collections/`, inspiré de Project
  NOMAD) et un outil de téléchargement intégré.
- **Messagerie par ondes radio et Bluetooth** — communication de proximité
  sans passer par internet ni par une antenne télécom, via
  [Reticulum](https://reticulum.network/) (RNS) et LXMF, sur des liaisons
  LoRa (longue portée, faible débit) et Bluetooth Low Energy. Une seule
  identité stable, avec relais en mode store-and-forward pour étendre la
  portée du réseau.
- **Salle de jeux** — émulateurs de consoles rétro et jeux pensés pour
  jouer contre une IA locale, à commencer par les échecs
  (moteur [Stockfish](https://stockfishchess.org/) en adversaire, via le
  protocole standard UCI).
- **Cartes hors ligne** — tuiles OSM locales via
  [ProtoMaps](https://protomaps.com/), sans dépendre d'un service de cartes
  en ligne.
- **Mode vocal** — dictée et synthèse vocale en local (whisper.cpp pour la
  reconnaissance, Piper pour la voix), activées au bouton (push-to-talk),
  jamais en écoute continue.

## Stack technique

| Domaine | Choix |
|---|---|
| Shell natif | [Tauri](https://tauri.app/) (Rust) |
| Interface | Svelte 5 + Vite + TypeScript, Tailwind CSS v4 |
| Orchestration des services | Docker Compose (un service = un conteneur) |
| LLM local | Ollama, famille Qwen3.5 |
| RAG / mémoire | Qdrant (vector store), fenêtre glissante de 30 jours |
| Académie | Kolibri |
| Contenus hors ligne | Kiwix |
| Cartes | ProtoMaps |
| Messagerie de proximité | Reticulum (RNS) + LXMF, LoRa + BLE |
| Salle de jeux | Émulateurs + moteurs de jeu locaux (échecs : python-chess + Stockfish) |
| Voix | whisper.cpp (STT) + Piper (TTS) |

Détail des décisions d'architecture et de leur justification dans
[`docs/decisions/`](docs/decisions/).

## Cible matérielle

Pensé pour tourner sur un PC grand public — i7, 32 Go de RAM, GPU
**intégré** (pas de carte graphique dédiée). L'inférence du LLM tourne en
CPU/iGPU ; aucun composant de la stack ne suppose de GPU dédié ni de
cluster.

## État du projet

Développement en cours, par étapes séquentielles (une intégration à la
fois, validée avant de passer à la suivante) — voir
[`TODO.md`](TODO.md) pour l'avancement détaillé et
[`CLAUDE.md`](CLAUDE.md) pour les règles et le séquencement du
développement.

## Développement

```bash
npm install
npm run tauri dev
```

Nécessite le toolchain Rust, les dépendances système de Tauri (voir
[tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)),
et Docker pour les services (`docker compose up -d <service>`). Copier
`.env.example` en `.env` pour adapter la configuration locale (ports,
modèle LLM...) — les valeurs par défaut fonctionnent telles quelles.

## Structure du projet

```
src/            Interface Svelte
src-tauri/      Backend Rust / shell Tauri
collections/    Catalogue de contenus téléchargeables (Kiwix, cours...)
docs/decisions/ Historique des décisions d'architecture
docker-compose.yml   Services locaux (Ollama, Qdrant, Kolibri, Kiwix...)
CLAUDE.md       Contexte et règles du projet
TODO.md         Roadmap et avancement
```
