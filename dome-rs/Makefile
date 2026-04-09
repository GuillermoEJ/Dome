.PHONY: help build release test clean fmt lint run install uninstall check all

help:
	@echo "DOME - Secure Password Vault"
	@echo ""
	@echo "Available commands:"
	@echo "  make build      - Build debug version (fast)"
	@echo "  make release    - Build optimized release (slow, small)"
	@echo "  make test       - Run all tests"
	@echo "  make test-crypto- Run only crypto tests"
	@echo "  make run        - Build and run debug version"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make fmt        - Format code (cargo fmt)"
	@echo "  make lint       - Check code with clippy"
	@echo "  make check      - Quick compile check"
	@echo "  make size       - Show binary size"
	@echo "  make doc        - Generate and open documentation"
	@echo "  make install    - Install to ~/.local/bin/dome"
	@echo "  make uninstall  - Remove installed binary"
	@echo "  make all        - Clean, format, lint, test, and build release"

build:
	@echo "Building debug version..."
	cargo build

release:
	@echo "Building optimized release..."
	cargo build --release
	@echo ""
	@echo "Binary location:"
	@echo "  ./target/release/dome"

test:
	@echo "Running all tests..."
	cargo test --lib

test-crypto:
	@echo "Running crypto tests..."
	cargo test crypto --lib

run: build
	@echo ""
	@echo "Running DOME..."
	./target/debug/dome

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Done!"

fmt:
	@echo "Formatting code..."
	cargo fmt

lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

size: release
	@echo ""
	@echo "Binary size:"
	@ls -lh target/release/dome*
	@echo ""
	@echo "Release info:"
	@file target/release/dome* || echo "(binary info unavailable)"

doc:
	@echo "Generating documentation..."
	cargo doc --open

check:
	@echo "Quick compile check..."
	cargo check

install: release
	@echo "Installing to ~/.local/bin/dome"
	@mkdir -p ~/.local/bin
	@cp target/release/dome ~/.local/bin/
	@chmod +x ~/.local/bin/dome
	@echo "✅ Done! Binary installed at ~/.local/bin/dome"
	@echo ""
	@echo "To use from anywhere, add to your shell profile:"
	@echo "  export PATH=~/.local/bin:$$PATH"

uninstall:
	@echo "Removing installed binary..."
	@rm -f ~/.local/bin/dome
	@echo "✅ Done!"

all: clean fmt lint test release
	@echo "✅ All tasks completed!"
	@echo "Everything complete!"
	@echo "Binary ready at: ./target/release/dome"
