# Makefile for elfinstaller

# Default target
all: build

# Build the project for the current platform
build:
	cargo build --release

# Install the binary to /usr/local/bin (requires sudo)
install:
	cargo build --release
	sudo cp target/release/elfinstaller /usr/local/bin/elfinstaller
	sudo chmod +x /usr/local/bin/elfinstaller

# Clean the project
clean:
	cargo clean

# Run tests
test:
	cargo test

# Build and create a tarball for the current platform
package:
	cargo build --release
	mkdir -p dist
	cp target/release/elfinstaller dist/
	HOST_ARCH=$(uname -m)
	if [ "$HOST_ARCH" = "x86_64" ]; then
		cd dist && tar czvf elfinstaller-amd64.tar.gz elfinstaller
	elif [ "$HOST_ARCH" = "aarch64" ]; then
		cd dist && tar czvf elfinstaller-arm64.tar.gz elfinstaller
	else
		cd dist && tar czvf elfinstaller-unknown.tar.gz elfinstaller
	fi
	cd ..

# Build for AMD64 (x86_64)
amd64-package:
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p dist
	cp target/x86_64-unknown-linux-gnu/release/elfinstaller dist/
	cd dist && tar czvf elfinstaller-amd64.tar.gz elfinstaller
	cd ..

# Build for ARM64 (aarch64)
arm64-package:
	cargo build --release --target aarch64-unknown-linux-gnu
	mkdir -p dist
	cp target/aarch64-unknown-linux-gnu/release/elfinstaller dist/
	cd dist && tar czvf elfinstaller-arm64.tar.gz elfinstaller
	cd ..

# Build a native Arch Linux package (.pkg.tar.zst) from the current checkout.
PKGVER := $(shell sed -n 's/^pkgver=//p' PKGBUILD)
CARGO_BUILD_JOBS ?= 1
arch-package:
	@command -v makepkg >/dev/null || { echo "makepkg is required (run this on Arch Linux)" >&2; exit 1; }
	@command -v cargo >/dev/null || { echo "cargo is required (install the rust package)" >&2; exit 1; }
	@mkdir -p dist
	@set -eu; \
		source_archive="elfinstaller-$(PKGVER).tar.gz"; \
		trap 'rm -f "$$source_archive" "src/elfinstaller-$(PKGVER).tar.gz"' EXIT; \
		rm -rf "src/elfinstaller-$(PKGVER)" pkg; \
		rm -f "src/elfinstaller-$(PKGVER).tar.gz"; \
		tar -czf "$$source_archive" --transform='s,^,elfinstaller-$(PKGVER)/,' \
			--exclude='src/elfinstaller-*' \
			Cargo.toml Cargo.lock LICENSE README.md src; \
		rm -f dist/elfinstaller-*.pkg.tar.*; \
		CARGO_BUILD_JOBS="$(CARGO_BUILD_JOBS)" PKGDEST="$(CURDIR)/dist" \
			makepkg --clean --force --nodeps --noconfirm

# Run the installed binary
run:
	@echo "Run the installed binary with: /usr/local/bin/elfinstaller --help"

.PHONY: all build install clean test package amd64-package arm64-package arch-package run
