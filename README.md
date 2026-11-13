# EternalLauncher

Le launcher de bureau des jeux EternalTwin, pour Windows, macOS et Linux.

> **En construction.** Aucune version n'est encore publiée.

## Ce qu'il fera

- Lister les jeux EternalTwin qui se jouent hors du navigateur. Le premier est [Galaxy 55](https://github.com/Pv1E/galaxy55-offline).
- Suivre les releases GitHub de chaque jeu et signaler les nouvelles versions. Le joueur choisit ce qu'il installe, ce qu'il garde et ce qu'il lance : rien ne s'installe sans lui.
- Se mettre à jour depuis les releases de ce dépôt, sans réinstallation, quand le joueur le demande.
- Permettre, sans l'imposer, de se connecter à son compte EternalTwin pour afficher son nom.

Interface en français, anglais, espagnol, allemand et portugais du Brésil.

## Développer

Le launcher est fait avec [Tauri 2](https://v2.tauri.app/) (Rust) et une interface Vue 3 + TypeScript.

Prérequis :
- [Node.js](https://nodejs.org/) 24 et [pnpm](https://pnpm.io/) ;
- [Rust](https://rustup.rs/) stable ;
- selon le système, les [prérequis de Tauri](https://v2.tauri.app/start/prerequisites/) :
  - Windows : Build Tools Visual Studio (C++) ;
  - Linux : `libwebkit2gtk-4.1-dev` et les paquets listés par Tauri ;
  - macOS : outils en ligne de commande de Xcode.

Commandes :

```sh
pnpm install          # dépendances
pnpm tauri dev        # lance le launcher en mode développement
pnpm test             # tests de l'interface
pnpm typecheck        # vérification des types
pnpm tauri build      # installeurs pour le système courant
```

À chaque pull request, la CI compile le launcher pour Windows, Linux et macOS. Elle joint les installeurs, non signés, à l'exécution correspondante de l'onglet *Actions* pendant 7 jours.

## Contribuer

La branche `main` est protégée : toute modification passe par une branche et une pull request.

- Les PR, les commits et les commentaires sont rédigés en français.
- Les commits suivent le format `<émoji> (<portée>) <message>`, par exemple `✨ (catalogue) liste des jeux` ou `🐛 (installation) refuse un zip corrompu`.
- Les titres de PR suivent le même principe : `✨ Ajout de …`, `🐛 Correction de …`.

## Licence

[GNU AGPL v3.0](LICENSE), comme les autres projets EternalTwin.

---

## English

EternalLauncher is the desktop launcher for EternalTwin games, on Windows, macOS and Linux. It is **under construction** and has no release yet.

It will:
- list the EternalTwin games that run outside the browser, starting with Galaxy 55;
- track each game's GitHub releases and let players choose which versions to install and launch (nothing installs on its own);
- update itself from this repository's releases when the player asks;
- optionally sign players in to their EternalTwin account.

Contributions go through pull requests; the `main` branch is protected. Licensed under the [GNU AGPL v3.0](LICENSE).
