SHELL := /bin/bash

CARGO := cargo
BIN := calculator_cli

.DEFAULT_GOAL := help

.PHONY: help build run release run-release test check fmt fmt-check clippy doc clean

help:
	@echo "Targets disponibles:"
	@echo "  make build        - Compile en mode debug"
	@echo "  make run          - Lance l'application (ARGS=\"...\" possible)"
	@echo "  make release      - Compile en mode release"
	@echo "  make run-release  - Lance le binaire release (ARGS=\"...\" possible)"
	@echo "  make test         - Lance les tests"
	@echo "  make check        - Verifie la compilation sans binaire"
	@echo "  make fmt          - Formate le code"
	@echo "  make fmt-check    - Verifie le formatage"
	@echo "  make clippy       - Lance clippy avec warnings en erreur"
	@echo "  make doc          - Genere la documentation"
	@echo "  make clean        - Nettoie les artefacts"

build:
	$(CARGO) build

run:
	$(CARGO) run -- $(ARGS)

release:
	$(CARGO) build --release

run-release: release
	./target/release/$(BIN) $(ARGS)

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

fmt-check:
	$(CARGO) fmt -- --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

doc:
	$(CARGO) doc --no-deps

clean:
	$(CARGO) clean
