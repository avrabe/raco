# RACO - Ralf's AI Code Orchestrator

RACO is a console and web-based AI agent for application development with human-in-the-loop workflows. It leverages the MCP (Model Context Protocol) to provide a flexible and powerful development environment.

## Features

- Console and web-based interfaces
- Human-in-the-loop workflows
- MCP integration with dynamically configurable servers
- Filesystem, Git, and Rust toolchain management
- Debugging support for host and QEMU Zephyr targets
- System analysis for optimal repository setup

## Project Status

This project is currently in early development.

## Requirements

- Rust 1.70 or later
- Just command runner
- Python 3.7 or later (for documentation)

## Getting Started

### Setup Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/raco.git
   cd raco
   ```

2. Install development tools:
   ```bash
   just setup
   ```

3. Build the project:
   ```bash
   just build
   ```

4. Run tests:
   ```bash
   just test
   ```

### Running RACO

#### CLI Interface

```bash
just run-cli --help
```

#### Web Interface

```bash
just run-web
```

## Architecture

RACO is built around a modular, extensible architecture:

- Core System
  - MCP Protocol Layer
  - Dynamic Server Manager
  - Workflow Engine
  - System Analyzer

- Development Environment
  - Filesystem and Git MCP Servers
  - Rust Toolchain MCP Server
  - Debugging MCP Servers (Host and QEMU Zephyr)

- User Interfaces
  - Console Interface
  - Web Interface

For detailed architecture information, please see the [documentation](docs/source/architecture.rst).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Format your code (`just fmt`)
4. Run clippy (`just clippy`)
5. Run tests (`just test`)
6. Commit your changes (`git commit -m 'Add some amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is dual-licensed under MIT or Apache-2.0 - see the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details. 