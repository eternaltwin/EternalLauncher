# Catalogue des jeux

`games.json` liste les jeux que propose EternalLauncher. Le launcher le lit au démarrage depuis la branche `main` de ce dépôt : ajouter ou corriger un jeu se fait par une pull request, sans publier de nouvelle version du launcher. Une copie est livrée avec chaque version du launcher, pour fonctionner hors ligne.

Le format est décrit par [`schema.json`](schema.json). Les tests Rust (`cargo test`, lancés par la CI) vérifient que le catalogue du dépôt est valide.

## Une entrée

| Champ | Rôle |
|---|---|
| `id` | Identifiant stable, en minuscules (`galaxy55`). Il nomme aussi le dossier d'installation : ne plus le changer une fois publié. |
| `kind` | `native` : le jeu se télécharge depuis ses releases GitHub, puis se lance. |
| `name`, `description` | Textes par langue : `fr`, `en`, `es`, `de`, `pt-BR`. Il en faut au moins une ; à défaut, le launcher prend l'anglais, puis le français. |
| `icon` | Icône carrée dans `icons/` (PNG, JPG, WebP ou SVG, 512 Ko au plus). |
| `minLauncher` | Facultatif : un launcher plus ancien ignore l'entrée. |
| `source.repo` | Dépôt GitHub du jeu, `propriétaire/nom`. |
| `source.minVersion` | Facultatif : les versions plus anciennes ne sont pas proposées. |
| `source.prereleases` | `true` pour proposer aussi les releases marquées *pre-release*. |
| `platforms` | Une entrée par système : `windows-x86_64`, `linux-x86_64`, `macos-aarch64`…, ou `linux-*` pour toute architecture. |
| `platforms.*.asset` | Nom du fichier de la release : `{tag}` vaut le tag GitHub (`v0.8`), `{version}` le tag sans le « v » (`0.8`). |
| `platforms.*.program`, `args`, `runtime`, `variants` | Comment lancer le jeu : programme du dossier de la version, arguments, runtime requis (`hashlink>=1.16`), variantes (DirectX…). |
| `saves` | Dossier des sauvegardes du jeu, par système. |

Une release n'est proposée que si elle contient l'asset de la plateforme du joueur. Les brouillons ne le sont jamais.
