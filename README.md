# Calculator CLI en Rust

Petit projet de calculatrice en ligne de commande pour pratiquer Rust:
- parsing d'expressions
- gestion des erreurs
- tests CLI
- historique persistant borné

## Fonctionnalités

- Calcul en mode argument direct
- Calcul en mode interactif (boucle de saisie)
- Opérateurs pris en charge: `+`, `-`, `*`, `/`, `%`, `^`, `!`
- Priorité des opérations:
1. factorielle `!`
2. puissance `^`
3. multiplication, division, modulo (`*`, `/`, `%`)
4. addition, soustraction (`+`, `-`)
- Historique enregistré dans `history.txt`
- Historique limité aux 100 dernières entrées (pour éviter une croissance infinie)

## Prérequis

- Rust (toolchain installée via `rustup`)
- Cargo

## Installation

Depuis la racine du projet:

```bash
cargo build
```

## Utilisation

### 1) Mode argument direct

```bash
cargo run -- 1 + 2
```

Exemple:

```text
3
```

### 2) Mode interactif

```bash
cargo run
```

Commandes disponibles:
- `help`: affiche l'aide
- `history`: affiche l'historique
- `quit`: quitte le programme

Exemple de session:

```text
> 5 * 4
= 20
> 3 !
= 6
> history
Ancien calcul : 5 * 4 = 20
Ancien calcul : 3 ! = 6
```

## Historique

- Fichier: `history.txt`
- Politique de conservation: uniquement les 100 dernières lignes
- Chaque ligne est stockée au format:

```text
expression = resultat
```

## Tests

Lancer tous les tests:

```bash
cargo test
```

Le projet contient des tests d'intégration CLI dans `tests/cli_tests.rs`.

## Commandes Make

Le projet inclut un `Makefile` avec des raccourcis:

```bash
make help
make build
make run ARGS="1 + 2"
make test
make check
make fmt
make clippy
make clean
```

## Structure du projet

```text
src/
	main.rs     # point d'entrée CLI
	lib.rs      # logique de parsing/evaluation
	types.rs    # types, enum d'operateurs et erreurs
	files.rs    # gestion de l'historique
tests/
	cli_tests.rs
```

