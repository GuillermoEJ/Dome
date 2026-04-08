.PHONY: help build release test clean fmt lint run install

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
	@echo "  make size       - Show binary size"
	@echo "  make doc        - Generate and open documentation"

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
	@echo "Binary sizes:"
	@ls -lh target/release/dome
	@echo ""
	@echo "Stripped size (Linux/macOS):"
	@strip -o /tmp/dome-stripped target/release/dome 2>/dev/null && ls -lh /tmp/dome-stripped || echo "(Windows - skip strip)"

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
	@echo "Done! Add to PATH if not already:"
	@echo "  export PATH=~/.local/bin:$$PATH"

all: clean fmt lint test release
	@echo ""
	@echo "✅ Everything complete!"
	@echo "Binary ready at: ./target/release/dome"
