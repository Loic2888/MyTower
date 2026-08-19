# 0003 — Ajouter Kiwix (Wikipedia/Wiktionnaire hors ligne) à la stack

Date : 2026-08-15

## Contexte

En testant Kolibri, il est apparu que remplir un outil éducatif de contenu
nécessite internet au moins une fois — logique (le contenu doit venir de
quelque part), mais l'utilisateur a demandé qu'on s'inspire de Project
NOMAD (déjà référencé dans `CLAUDE.md` comme référence d'architecture) pour
rendre ce moment de provisionnement aussi simple et regroupé que possible :
un dossier `collections/` cataloguant du contenu prêt à télécharger, et un
outil "Contenus" (Easy Setup Wizard) pour le faire en un clic.

Un des piliers de NOMAD pour l'accès à la connaissance hors ligne est
**Kiwix** — un serveur qui sert des archives Wikipedia/Wiktionnaire/autres
au format `.zim`, entièrement hors ligne une fois le fichier `.zim` obtenu.
Ce n'était pas dans la stack canonique de `CLAUDE.md`.

## Décision

Ajout de **Kiwix** (`ghcr.io/kiwix/kiwix-serve`) à la stack, au même titre
que Kolibri/ProtoMaps : un service Docker de plus, dédié au contenu
encyclopédique hors ligne.

Détails vérifiés (comme pour Kolibri, pas de config devinée) :
- Image officielle `ghcr.io/kiwix/kiwix-serve`, sert les `.zim` d'un
  dossier monté (`command: ["*.zim"]` = charge tout fichier `.zim` présent).
- Port 8080 par défaut à l'intérieur du conteneur — **remappé sur 8082**
  côté hôte (`KIWIX_PORT`) car 8080 est déjà pris par Kolibri.
- `kiwix-data` monté en **bind-mount** (`./data/kiwix`), pas en volume
  Docker nommé comme les autres services : le backend Rust (nouvelle
  commande `download_zim`, `src-tauri/src/wizard.rs`) doit pouvoir écrire
  directement les fichiers `.zim` téléchargés dans ce dossier.
- Tailles de fichiers réelles vérifiées sur le miroir officiel
  `lb.download.kiwix.org` avant de les proposer dans
  `collections/wikipedia.json` — pas de tag/fichier inventé.

## Bugs trouvés et corrigés pendant le test réel

- **Dossier courant du process au runtime** : `cargo tauri dev` lance le
  binaire avec `src-tauri/` comme dossier courant, pas la racine du repo
  (vérifié via `/proc/<pid>/cwd`). `dotenvy::dotenv()` (qui cherche `.env`
  dans le dossier courant) ne l'aurait donc **jamais trouvé** à la racine où
  il est censé vivre, et `KIWIX_DATA_DIR=./data/kiwix` (valeur relative) se
  serait résolu vers `src-tauri/data/kiwix` au lieu de `data/kiwix` à la
  racine — deux dossiers différents du bind-mount Docker. Corrigé avec
  `src-tauri/src/paths.rs` (`project_root()`, ancré sur `CARGO_MANIFEST_DIR`
  à la compilation) : `.env` est maintenant chargé explicitement depuis la
  racine, et `KIWIX_DATA_DIR` relatif est résolu depuis la racine, pas le
  dossier courant.
- **`kiwix-serve` refuse de démarrer si aucun `.zim` n'est présent** : avec
  un dossier `data/kiwix/` vide, `command: ["*.zim"]` ne s'étend pas (pas de
  shell dans le conteneur) et le conteneur quitte immédiatement. Normal —
  mais donc `docker compose up -d kiwix` seul, sans contenu déjà téléchargé,
  ne suffit pas à avoir un serveur qui tourne.
- **`kiwix-serve` ne détecte pas un `.zim` ajouté après son démarrage** :
  vérifié en pratique — il a fallu `docker compose restart kiwix` après le
  téléchargement pour que le fichier soit servi. À prendre en compte côté
  UI plus tard (proposer de redémarrer le service après un téléchargement).
- **Le bind-mount `./data/kiwix` est créé par Docker (root) s'il n'existe
  pas encore** : `docker compose up` avant tout téléchargement crée le
  dossier appartenant à `root`, ce qui aurait bloqué en écriture aussi bien
  `curl` en test manuel que la commande Rust `download_zim` en usage réel.
  Corrigé pour ce test via un conteneur Alpine jetable
  (`docker run --rm -v ./data:/data alpine chown -R <uid>:<gid> /data`) —
  pas une correction permanente dans le code, juste un contournement
  ponctuel. Si ça se reproduit (ex. sur une autre machine), le même
  contournement s'applique.

## Conséquences

- `data/` (contenu téléchargé) ajouté au `.gitignore` — jamais versionné,
  potentiellement plusieurs Go.
- Le dossier `collections/` (à la racine, comme NOMAD) et l'outil
  "Contenus" dans `ToolsPanel` sont réutilisables pour d'autres sources de
  contenu plus tard (ex. ProtoMaps, quand son étape démarrera).
- Pas de petit fichier Wikipedia en français disponible (le plus petit fait
  3,3 Go) — assumé et affiché tel quel dans le catalogue plutôt que caché.
- Cette extension anticipe partiellement l'étape 3 du séquencement
  (Kiwix n'était l'objet d'aucune étape prévue) — décision explicite de
  l'utilisateur, comme documenté dans `TODO.md`.
