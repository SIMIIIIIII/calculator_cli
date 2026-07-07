# Calculator CLI en Rust

Calculatrice en ligne de commande écrite en Rust pour pratiquer :
- parsing d'expressions arithmétiques avec arbre binaire
- gestion des erreurs typées
- tests unitaires et d'intégration CLI
- historique persistant borné

## Fonctionnalités

- Calcul en mode argument direct
- Calcul en mode interactif (boucle de saisie)
- Support des parenthèses imbriquées : `((-2 * 3) / 2)`
- Opérateurs supportés :

| Symbole | Opération        | Exemple       |
|---------|-----------------|---------------|
| `+`     | Addition         | `5 + 3` → `8` |
| `-`     | Soustraction     | `5 - 3` → `2` |
| `*`     | Multiplication   | `4 * 3` → `12`|
| `/`     | Division         | `9 / 3` → `3` |
| `%`     | Modulo           | `7 % 3` → `1` |
| `^`     | Puissance        | `2 ^ 8` → `256`|
| `!`     | Factorielle      | `5 !` → `120` |

- Historique enregistré dans `history.txt`, limité aux 100 dernières entrées

## Prérequis

- Rust (toolchain installée via `rustup`)
- Cargo

## Installation

```bash
cargo build
```

## Utilisation

### Mode argument direct

```bash
cargo run -- 5 + 3
# ou avec des parenthèses (guillemets nécessaires dans le shell)
cargo run -- "(2 ^ 3) * 2"
```

### Mode interactif

```bash
cargo run
```

Commandes disponibles :
- `help` — affiche l'aide
- `history` — affiche l'historique des calculs
- `quit` — quitte le programme

Exemple de session :

```text
> 5 * 4
20
> (2 ^ 2 + 1) !
120
> 7 % 3
1
> history
5 * 4 = 20
(2 ^ 2 + 1) ! = 120
7 % 3 = 1
```

## Gestion des erreurs

| Erreur                  | Déclenchement                              |
|-------------------------|--------------------------------------------|
| `EmptyInput`            | Expression vide                            |
| `InvalidFormat`         | Syntaxe incorrecte                         |
| `InvalidNumber`         | Token non parseable comme nombre           |
| `InvalidOperator`       | Caractère non reconnu comme opérateur      |
| `DivisionByZero`        | Diviseur ou modulo égal à zéro             |
| `NegativeNumber`        | Factorielle d'un nombre négatif            |
| `DecimalNumber`         | Factorielle d'un nombre décimal            |
| `Overflow`              | Résultat trop grand (ex: `21!`)            |
| `ExpressionConstruction`| Construction d'arbre invalide              |
| `InvalidOperation`      | Opération interne invalide                 |

## Historique

- Fichier : `history.txt`
- Format de chaque ligne : `expression = résultat`
- Politique : uniquement les 100 dernières entrées conservées

## Tests

```bash
cargo test
```

- `tests/expression_tests.rs` — tests unitaires sur le parsing et l'évaluation
- `tests/cli_tests.rs` — tests d'intégration sur le binaire compilé

## Commandes Make

```bash
make build        # Compile en mode debug
make run ARGS="5 + 3"  # Lance avec arguments
make release      # Compile en mode release
make test         # Lance les tests
make check        # Vérifie la compilation sans produire de binaire
make fmt          # Formate le code
make clippy       # Lance clippy
make doc          # Génère la documentation
make clean        # Nettoie les artefacts
```

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

