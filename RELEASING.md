# Publier une version du launcher

Les joueurs reçoivent les nouvelles versions par le launcher lui-même : il regarde la dernière release publiée de ce dépôt, affiche un bandeau, et n'installe rien tant que le joueur n'a pas cliqué sur « Installer et redémarrer ».

Chaque mise à jour est **signée**. Le launcher refuse un fichier qui n'est pas signé par la clé du projet, même s'il vient de GitHub.

## Une seule fois : la clé de signature

La clé a été générée avec `pnpm tauri signer generate`. Elle comprend trois fichiers :

| Fichier | Contenu | Où le ranger |
|---|---|---|
| `eternallauncher.key.pub` | clé publique | déjà dans `src-tauri/tauri.conf.json` (`plugins.updater.pubkey`) |
| `eternallauncher.key` | clé privée, chiffrée par le mot de passe | secret GitHub `TAURI_SIGNING_PRIVATE_KEY` + gestionnaire de mots de passe |
| `eternallauncher.password` | mot de passe de la clé privée | secret GitHub `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` + gestionnaire de mots de passe |

Pour poser les secrets :
1. Sur le dépôt, aller dans **Settings → Secrets and variables → Actions → New repository secret**.
2. Créer `TAURI_SIGNING_PRIVATE_KEY` avec **tout le contenu** du fichier `eternallauncher.key`.
3. Créer `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` avec le contenu de `eternallauncher.password`.

**Ne jamais perdre la clé privée.** Sans elle, aucune version future ne peut être signée, et chaque joueur devrait réinstaller le launcher à la main. Gardez-en une copie hors de GitHub, par exemple dans un gestionnaire de mots de passe. Ne la commitez jamais.

## À chaque version

1. Sur une branche, monter la version au même numéro dans `package.json` et `src-tauri/Cargo.toml` (un test vérifie qu'ils sont égaux), puis compléter `CHANGELOG.md`. Ouvrir la PR et la fusionner.
2. Sur `main` à jour, créer et pousser le tag :

   ```sh
   git switch main
   git pull
   git tag v0.2.0
   git push origin v0.2.0
   ```

3. Le workflow **Release** vérifie que le tag correspond à la version. Il construit ensuite les installeurs signés pour Windows, Linux, macOS Apple Silicon et macOS Intel, et crée une release **brouillon** avec `latest.json`.
4. Relire le brouillon dans **Releases** : les installeurs sont présents, les `.sig` aussi, et `latest.json` cite les 4 plateformes. Puis cliquer **Publish release**.
5. À partir de là, les launchers installés proposent la mise à jour, au démarrage ou par « Rechercher une mise à jour » dans les Réglages.

Une release marquée *pre-release* n'est pas proposée aux joueurs : le launcher ne lit que la dernière release publiée.

## Installeurs non signés

Les installeurs ne sont pas encore signés par un certificat Windows ni Apple. À la première installation :
- Windows affiche « Windows a protégé votre ordinateur » : cliquer sur **Informations complémentaires**, puis **Exécuter quand même** ;
- macOS demande d'aller dans **Réglages Système → Confidentialité et sécurité → Ouvrir quand même**.

Les mises à jour suivantes passent par le launcher lui-même, sans ces écrans.
