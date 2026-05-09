SHELL := /bin/sh

WINDOWS_TARGET ?= x86_64-pc-windows-gnu
LINUX_BIN := ./target/release/rust_browser_handler

.PHONY: help setup-dev run run-windows dev test test-watch format lint check-all build-linux build-windows install register uninstall

help:
	@echo "Rust Browser Handler - Makefile shortcuts"
	@echo ""
	@echo "Setup"
	@echo "  make setup-dev         # cargo setup-dev"
	@echo ""
	@echo "Development"
	@echo "  make run               # cargo run"
	@echo "  make run-windows       # cargo run --target $(WINDOWS_TARGET)"
	@echo "  make dev               # cargo dev"
	@echo "  make test              # cargo test"
	@echo "  make test-watch        # cargo test-watch"
	@echo "  make format            # cargo format"
	@echo "  make lint              # cargo lint"
	@echo "  make check-all         # cargo check-all"
	@echo ""
	@echo "Build"
	@echo "  make build-linux       # cargo build --release"
	@echo "  make build-windows     # cargo build --release --target $(WINDOWS_TARGET)"
	@echo ""
	@echo "Linux registration"
	@echo "  make install           # target/release/rust_browser_handler install"
	@echo "  make register          # target/release/rust_browser_handler register"
	@echo "  make uninstall         # target/release/rust_browser_handler uninstall"

setup-dev:
	cargo setup-dev

run:
	cargo run

run-windows:
	cargo run --target $(WINDOWS_TARGET)

dev:
	cargo dev

test:
	cargo test

test-watch:
	cargo test-watch

format:
	cargo format

lint:
	cargo lint

check-all:
	cargo check-all

build-linux:
	cargo build --release

build-windows:
	cargo build --release --target $(WINDOWS_TARGET)

install:
	if [ ! -f $(LINUX_BIN) ]; then $(MAKE) build-linux; fi
	$(LINUX_BIN) install

register:
	if [ ! -f $(LINUX_BIN) ]; then $(MAKE) build-linux; fi
	$(LINUX_BIN) register

uninstall:
	if [ ! -f $(LINUX_BIN) ]; then $(MAKE) build-linux; fi
	$(LINUX_BIN) uninstall
