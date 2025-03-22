# RACO development tasks

# Default task shows help
default:
    @just --list

# Setup development environment
setup:
    cargo install cargo-watch
    cargo install cargo-expand
    cargo install cargo-sphinx
    cargo install cargo-tarpaulin
    cargo install sphinx-needs-builders
    echo "Setup complete. You may need to install Python dependencies:"
    echo "pip install -r docs/requirements.txt"

# Run tests with coverage
test:
    cargo tarpaulin --workspace --exclude-files "*/tests/*" --out xml

# Build project
build:
    cargo build --workspace

# Run the CLI application
run-cli *ARGS:
    cargo run -p raco-cli -- {{ARGS}}

# Run the web application
run-web:
    cargo run -p raco-web

# Watch for changes and run tests
watch-test:
    cargo watch -x "test --workspace"

# Watch for changes and run build
watch-build:
    cargo watch -x "build --workspace"

# Format code
fmt:
    cargo fmt --all

# Check code with clippy
clippy:
    cargo clippy --workspace --all-features -- -D warnings

# Clean build artifacts
clean:
    cargo clean
    rm -rf docs/_build

# Create a new release
release VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Creating release v{{VERSION}}..."
    git tag -a "v{{VERSION}}" -m "Release v{{VERSION}}"
    git push origin "v{{VERSION}}"
    echo "Release v{{VERSION}} created and pushed."

# ----------------- Code Quality Commands -----------------

# Check code style
check:
    cargo fmt -- --check
    cargo clippy --workspace -- -D warnings

# Check for unused dependencies
check-udeps:
    #!/usr/bin/env bash
    # Check if we have nightly toolchain
    if ! rustup toolchain list | grep -q "nightly"; then
        echo "Nightly toolchain required for cargo-udeps. Installing..."
        rustup toolchain install nightly --profile minimal
    fi
    
    # Check if we're on Windows
    if [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* || "$OSTYPE" == "win"* ]]; then
        echo "Running unused dependencies check on Windows..."
        # On Windows, just check that cargo-udeps is available but don't run it in CI
        # This avoids some Windows-specific nightly toolchain issues
        if command -v cargo-udeps &> /dev/null || rustup run nightly cargo udeps --version &> /dev/null; then
            echo "cargo-udeps is available. Skipping actual check on Windows for compatibility."
            echo "Note: For thorough checking, run 'cargo +nightly udeps' manually."
            exit 0
        else
            echo "Installing cargo-udeps..."
            cargo install cargo-udeps --locked || echo "Failed to install cargo-udeps, but continuing..."
            echo "cargo-udeps installation attempted. Skipping actual check on Windows for compatibility."
            exit 0
        fi
    else
        # Unix platforms (Linux, macOS)
        echo "Running unused dependencies check on Unix platform..."
        # Ensure cargo-udeps is installed
        if ! command -v cargo-udeps &> /dev/null; then
            echo "Installing cargo-udeps..."
            cargo install cargo-udeps --locked || echo "Warning: Failed to install cargo-udeps"
        fi
        
        # Run the actual check
        echo "Checking for unused dependencies..."
        cargo +nightly udeps --workspace --all-targets || echo "Note: Some dependencies may be required for future development"
    fi

# Run all checks (format, clippy, tests)
check-all: check test check-udeps
    @echo "✅ All checks passed! Code is ready to commit."

# ----------------- Documentation Commands -----------------

# Variables for Sphinx documentation
sphinx_opts := ""
sphinx_build := "sphinx-build"
sphinx_source := "docs/source"
sphinx_build_dir := "docs/_build"

# Build HTML documentation
docs-html:
    {{sphinx_build}} -M html "{{sphinx_source}}" "{{sphinx_build_dir}}" {{sphinx_opts}}
    
# Build HTML documentation with PlantUML diagrams
docs-with-diagrams: setup-python-deps setup-plantuml
    #!/usr/bin/env bash
    # Set PLANTUML_PATH environment variable
    if command -v plantuml &> /dev/null; then
        export PLANTUML_PATH="$(which plantuml)"
        echo "Using PlantUML from: $PLANTUML_PATH"
    else
        echo "WARNING: PlantUML not found in PATH. Using default 'plantuml' command."
    fi
    
    # First clean any previous diagrams
    echo "Cleaning previous diagram build artifacts..."
    rm -rf "{{sphinx_build_dir}}/html/_images/plantuml-*" "{{sphinx_build_dir}}/html/_plantuml" || true
    
    # Generate changelog if git-cliff is available 
    if command -v git-cliff &> /dev/null; then
        echo "Generating changelog..."
        git-cliff -o docs/source/changelog.md
    fi

    # Build with PlantUML diagrams
    echo "Building documentation with PlantUML diagrams..."
    {{sphinx_build}} -M html "{{sphinx_source}}" "{{sphinx_build_dir}}" {{sphinx_opts}}
    
    # Confirm diagrams were generated
    DIAGRAM_COUNT=$(find "{{sphinx_build_dir}}/html/_images" -name "plantuml-*" | wc -l)
    echo "Generated $DIAGRAM_COUNT PlantUML diagrams"
    
    if [ "$DIAGRAM_COUNT" -eq 0 ]; then
        echo "WARNING: No PlantUML diagrams were generated. There might be an issue with the PlantUML setup."
        echo "Check that your .puml files are properly formatted and the PlantUML executable is available."
    fi

# Build PDF documentation (requires LaTeX installation)
docs-pdf:
    {{sphinx_build}} -M latex "{{sphinx_source}}" "{{sphinx_build_dir}}" {{sphinx_opts}}
    @echo "LaTeX files generated in docs/_build/latex. Run 'make' in that directory to build PDF (requires LaTeX installation)."

# Build all documentation formats (HTML with diagrams by default)
docs: docs-with-diagrams
    @echo "Documentation built successfully. HTML documentation available in docs/_build/html."
    @echo "To build PDF documentation, run 'just docs-pdf' (requires LaTeX installation)."

# Show Sphinx documentation help
docs-help:
    {{sphinx_build}} -M help "{{sphinx_source}}" "{{sphinx_build_dir}}" {{sphinx_opts}}

# ----------------- Utility Commands -----------------

# Install rust targets required for the project
setup-rust-targets:
    #!/usr/bin/env bash
    echo "Installing required Rust targets..."

    # Install the default Rust target for the host system
    rustup target add wasm32-unknown-unknown || { echo "Failed to install wasm32-unknown-unknown target."; exit 1; }

    echo "Rust targets installed successfully!"

# Install Python dependencies
setup-python-deps:
    cargo install git-cliff || echo "git-cliff already installed or installation failed"
    pip install -r docs/requirements.txt

# Install PlantUML (requires Java)
setup-plantuml:
    #!/usr/bin/env bash
    if ! command -v plantuml &> /dev/null; then
        echo "Installing PlantUML..."
        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            # Linux installation
            sudo apt-get update && sudo apt-get install -y plantuml
            # Set path for Linux
            export PLANTUML_PATH="$(which plantuml)"
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS installation with Homebrew
            if command -v brew &> /dev/null; then
                brew install plantuml
                # Set path for macOS
                export PLANTUML_PATH="$(which plantuml)"
            else
                echo "Homebrew not found. Please install homebrew first or install plantuml manually."
                echo "Visit: https://plantuml.com/starting"
                exit 1
            fi
        elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* || "$OSTYPE" == "win"* ]]; then
            # Windows installation
            echo "For Windows, please download PlantUML jar manually from https://plantuml.com/download"
            echo "Then set PLANTUML_PATH environment variable to the jar file path"
            echo "Example: set PLANTUML_PATH=C:\\path\\to\\plantuml.jar"
            exit 1
        else
            echo "Unsupported OS. Please install plantuml manually."
            echo "Visit: https://plantuml.com/starting"
            exit 1
        fi
    else
        echo "PlantUML is already installed."
        # Set path for installed PlantUML
        export PLANTUML_PATH="$(which plantuml)"
    fi
    
    # Check if Java is installed (required for PlantUML)
    if ! command -v java &> /dev/null; then
        echo "Java is required for PlantUML but not found. Please install Java."
        exit 1
    fi
    
    # Verify PlantUML is working by testing a simple diagram
    echo -e "@startuml\nBob -> Alice : hello\n@enduml" > /tmp/test.puml
    if ! plantuml /tmp/test.puml; then
        echo "WARNING: PlantUML installation test failed. Please verify your installation."
        exit 1
    fi
    echo "PlantUML test successful!"

# Install git hooks to enforce checks before commit/push
setup-hooks:
    @echo "Setting up Git hooks..."
    if [ -d .githooks ]; then
    cp .githooks/pre-commit .git/hooks/pre-commit
    cp .githooks/pre-push .git/hooks/pre-push
    chmod +x .git/hooks/pre-commit .git/hooks/pre-push
    echo "Git hooks installed successfully. Checks will run automatically before each commit and push."
    else
    echo "No .githooks directory found. Skipping hook installation."
    fi

# Show help
help:
    @just --list 