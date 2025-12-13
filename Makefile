MKFILE_PATH := $(patsubst %/,%, $(dir $(realpath $(firstword $(MAKEFILE_LIST)))))
DATE := $(shell date +%Y-%m-%d)

.DEFAULT_GOAL := help

check_and_install_rust:
	@if command -v rustc >/dev/null 2>&1; then \
		echo "Rust is already installed"; \
	else \
		echo "Rust is not installed"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		export PATH="$HOME/.cargo/bin:$PATH"; \
		echo "Rust has been installed"; \
	fi

build: check_and_install_rust
	cargo build

build-release: check_and_install_rust
	cargo build --release

clean:
	cargo clean

update:
	cargo update

test:
	cargo test --test binary_integration_test -- --nocapture

help:
	@echo "build : Build debug binary"
	@echo "build-release : Build release binary"
	@echo "clean : Clean cargo"
	@echo "update : Update cargo packages"
	@echo "test : Startet cargo test mit println Ausgabe"