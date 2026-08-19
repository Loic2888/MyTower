# 0004 — Retrait de CyberChef, salle de jeux élargie aux émulateurs

Date : 2026-08-16

## Contexte

Recentrage du périmètre produit décidé par l'utilisateur : CyberChef
(crypto/utilitaires) sort du projet, et la salle de jeux ne se limite plus
aux échecs — elle doit aussi accueillir des émulateurs (consoles rétro) en
plus des jeux programmés pour affronter une IA locale (les échecs restent
le premier de cette catégorie, inchangé : `python-chess` + Stockfish en
adversaire UCI).

Le reste du périmètre ne change pas : chat IA, Académie (Kolibri) +
Contenus (Kiwix), messagerie par ondes radio et Bluetooth (Reticulum/LXMF,
transports LoRa + BLE — déjà la description exacte de `CLAUDE.md` avant
cette décision, juste reformulée en langage courant par l'utilisateur).

## Décision

- **CyberChef retiré** de la stack canonique et du rail d'outils
  (`ToolsPanel.svelte`). Ajouté à la liste "Explicitement écarté" de
  `CLAUDE.md` — ne pas le réintroduire sans repasser par une décision.
- **Salle de jeux élargie** : émulateurs + jeux programmés contre IA locale,
  pas seulement les échecs. Le choix technique des émulateurs (quel(s)
  système(s) émulé(s), quelle bibliothèque) n'est pas encore tranché — à
  décider quand l'étape "salle de jeux" du séquencement démarrera
  réellement, pas anticipé ici.

## Conséquences

- `CLAUDE.md` (Stack canonique, séquencement, "Explicitement écarté") mis à
  jour.
- `ToolsPanel.svelte` : entrée CyberChef retirée du rail d'icônes ;
  `ToolIcon.svelte` : icône `cyberchef` retirée.
- `TODO.md` : item CyberChef retiré de l'étape 3, item salle de jeux mis à
  jour pour mentionner les émulateurs.
- `README.md` réécrit pour refléter ce périmètre.
- Le choix des émulateurs reste ouvert — pas de nouvelle dépendance ajoutée
  à `Cargo.toml`/`package.json`/`docker-compose.yml` pour l'instant, cette
  décision documente juste le changement de scope, pas une implémentation.
