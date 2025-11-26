# LLM Architect

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**LLM Architect** is a robust CLI tool designed to engineer high-fidelity prompts for Large Language Models. It bridges the gap between vague project ideas and professional-grade software specifications by wrapping input in strict engineering constraints, context-aware references, and architectural patterns.

## 🚀 Features

* **Multi-Mode Prompting**: Generate specialized prompts for different stages of the engineering lifecycle:
    * `Architecture`: Full system design and implementation planning.
    * `CodeReview`: Security, performance, and anti-pattern analysis.
    * `Refactor`: Modernization and technical debt reduction strategies.
    * `Readme`: Automatic documentation generation based on code analysis.
* **Context Injection**: Automatically scan (`--scan`) local directories to inject actual file structures and content, grounding LLM responses in your existing codebase.
* **Stack Awareness**: Tailors prompt constraints and best practices based on the specified technology stack (e.g., Rust, React, Python).
* **Pipeline Ready**: Supports standard input (`--stdin`) for seamless integration with other terminal tools.
* **Code Context**: Ability to scan and reference specific files or entire directories to provide the LLM with immediate context.

## 🛠️ Tech Stack

* **Language**: Rust (2024 Edition)
* **CLI Parsing**: `clap` (v4.5)
* **Error Handling**: `anyhow`
* **Context Scanning**: `code_context`

## 📦 Installation

Ensure you have Rust and Cargo installed.

```bash
# Clone the repository
git clone [https://github.com/yourusername/llm_architect.git](https://github.com/yourusername/llm_architect.git)
cd llm_architect

# Build the project
cargo build --release
```

The binary will be available in `./target/release/llm_architect`.

## 💻 Usage

The tool prints the generated prompt to `stdout`, allowing you to redirect output to files or copy it directly.

### 1. Architecture Design (Default)
Generate a comprehensive implementation plan for a new idea:

```bash
cargo run -- --description "A distributed task queue system using Redis" --stack "Go"
```

### 2. Code Review
Scan a directory and generate a review prompt for an existing project:

```bash
cargo run -- \
  --mode code-review \
  --scan ./src \
  --stack "Rust" \
  --description "Audit for concurrency bugs and memory leaks"
```

### 3. Refactoring
Pipe a legacy file into the tool to generate a modernization plan:

```bash
cat legacy_controller.js | cargo run -- \
  --stdin \
  --mode refactor \
  --stack "Node.js" \
  --description "Convert callbacks to async/await"
```

### 4. Injecting Constraints
Add specific requirements to the prompt:

```bash
cargo run -- \
  -d "Real-time chat app" \
  -s "React + Firebase" \
  --context "Must use Tailwind CSS and Firestore security rules"
```

## ⚙️ CLI Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--description` | `-d` | The main project idea or task description. | `None` |
| `--stack` | `-s` | Target programming language or tech stack. | "General Software" |
| `--context` | `-c` | Specific constraints or library requirements. | `None` |
| `--scan` | | Path to directory to scan for code context. | `None` |
| `--stdin` | | Read description from standard input. | `false` |
| `--mode` | `-m` | Generation mode: `architecture`, `code-review`, `refactor`, `readme`. | `architecture` |

## 🏗️ Architecture

The project follows a modular structure to ensure maintainability:

* `src/main.rs`: Minimal entry point.
* `src/app.rs`: Application orchestration and flow control.
* `src/app/cli.rs`: CLI argument parsing and `PromptMode` definitions.
* `src/app/generator.rs`: Logic for constructing prompts based on selected modes.
* `src/app/context.rs`: Wrapper for directory scanning functionality.

## 📄 License

This project is licensed under the MIT License.
