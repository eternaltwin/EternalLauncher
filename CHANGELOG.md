# Changelog

Toutes les évolutions notables d'EternalLauncher. Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/), et les versions [le versionnage sémantique](https://semver.org/lang/fr/).

## [Non publié]

### Sécurité

- Une mise à jour du launcher doit être signée pour sa propre version.
- La fenêtre du launcher n'affiche plus que sa propre interface : un lien ouvert dans une nouvelle fenêtre part dans le navigateur.

### Corrigé

- Un téléchargement ne s'interrompt plus au bout de 30 secondes sur une connexion lente.
- Supprimer une version dont un fichier est encore ouvert la laisse entière au lieu de la laisser à moitié effacée, et le reste d'une suppression interrompue n'empêche plus de la réinstaller.

## [0.1.0] - 2026-11-14

Première version.

### Ajouté

- Application de bureau pour Windows, macOS et Linux, en français, anglais, espagnol, allemand et portugais du Brésil.
- Catalogue des jeux lu sur ce dépôt. Premier jeu : Galaxy 55.
- Liste des jeux en lignes ou en grille, et fiche de chaque jeu avec ses versions publiées sur GitHub et leurs notes.
- Installation d'une version, vérifiée par son empreinte sha256, avec progression et annulation, puis lancement et désinstallation.
- Proposition de mise à jour quand une version plus récente que celles installées existe, et avertissement avant de lancer une version plus ancienne que la dernière jouée.
- Lancements alternatifs (DirectX pour Galaxy 55), à activer dans les Réglages.
- Mise à jour du launcher lui-même depuis les releases de ce dépôt : signée, et installée seulement quand le joueur le demande.

[Non publié]: https://github.com/eternaltwin/EternalLauncher/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/eternaltwin/EternalLauncher/releases/tag/v0.1.0
