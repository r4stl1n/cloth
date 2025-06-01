# Cloth CLI

A simple command-line interface application built in Rust for processing llm patterns with the openwebui api

## Features

- **Pattern Management**: Work with different patterns including extract_wisdom, humanize, and summarize
- **OpenWebUI Integration**: Seamless integration with OpenWebUI

## Installation

### Prerequisites

- Rust 1.87.0 or later
- Cargo package manager

### Build from Source

```shell script
git clone https://github.com/r4stl1n/cloth
cd cloth
cargo build --release
```

The binary will be available at `target/release/cloth`.

## Usage

# Command-Line Help for `cloth`

This document contains the help content for the `cloth` command-line program.

**Command Overview:**

* [`cloth`↴](#cloth)
* [`cloth config`↴](#cloth-config)
* [`cloth config view`↴](#cloth-config-view)
* [`cloth config setup`↴](#cloth-config-setup)
* [`cloth owui`↴](#cloth-owui)
* [`cloth owui list-models`↴](#cloth-owui-list-models)
* [`cloth owui completion`↴](#cloth-owui-completion)
* [`cloth pattern`↴](#cloth-pattern)
* [`cloth pattern list`↴](#cloth-pattern-list)
* [`cloth pattern view`↴](#cloth-pattern-view)
* [`cloth pattern process`↴](#cloth-pattern-process)
* [`cloth pattern raw`↴](#cloth-pattern-raw)

## `cloth`

**Usage:** `cloth <COMMAND>`

###### **Subcommands:**

* `config` — Config related command
* `owui` — OpenWebUi related commands
* `pattern` — Pattern-related commands

## `cloth config`

Config related command

**Usage:** `cloth config <COMMAND>`

###### **Subcommands:**

* `view` — Output the config file
* `setup` —

## `cloth config view`

Output the config file

**Usage:** `cloth config view`

## `cloth config setup`

**Usage:** `cloth config setup`

## `cloth owui`

OpenWebUi related commands

**Usage:** `cloth owui <COMMAND>`

###### **Subcommands:**

* `list-models` — List all installed models
* `completion` — Process a completion

## `cloth owui list-models`

List all installed models

**Usage:** `cloth owui list-models`

## `cloth owui completion`

Process a completion

**Usage:** `cloth owui completion [OPTIONS] --model <MODEL>`

###### **Options:**

* `--model <MODEL>` — Name of the model
* `--query <QUERY>` — Query to complete

## `cloth pattern`

Pattern-related commands

**Usage:** `cloth pattern [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all patterns
* `view` — View a specific pattern
* `process` — Process a pattern
* `raw` — Process a raw query

###### **Options:**

* `--pattern-directory <PATTERN_DIRECTORY>`

## `cloth pattern list`

List all patterns

**Usage:** `cloth pattern list`

## `cloth pattern view`

View a specific pattern

**Usage:** `cloth pattern view --pattern <PATTERN>`

###### **Options:**

* `--pattern <PATTERN>` — Name of the pattern

## `cloth pattern process`

Process a pattern

**Usage:** `cloth pattern process [OPTIONS] --name <NAME>`

###### **Options:**

* `--model <MODEL>` — Name of the model
* `--name <NAME>` — Name of the pattern
* `--query <QUERY>` — Query to process

## `cloth pattern raw`

Process a raw query

**Usage:** `cloth pattern raw [OPTIONS] --prompt <PROMPT>`

###### **Options:**

* `--model <MODEL>` — Name of the model
* `--prompt <PROMPT>` — Prompt
* `--query <QUERY>` — Query to process

### Pattern Directory Structure

The application comes with predefined patterns:

```
patterns/
├── extract_wisdom/
│   └── pattern.md
├── humanize/
│   └── pattern.md
└── summarize/
│   └── pattern.md
```

## Dependencies

- **clap**: Command-line argument parsing
- **serde**: Serialization/deserialization
- **tracing**: Structured logging
- **ureq**: HTTP client
- **eyre**: Error handling
- **dirs**: Directory utilities

## Development

### Project Structure

```
src/
├── commands/          # Command implementations
├── integrations/      # External service integrations
├── managers/          # Core business logic managers
├── utils/            # Utility functions
└── main.rs           # Application entry point
```

### Logging

The application uses structured logging with tracing. Log level is set to INFO by default.

## License

See [LICENSE](LICENSE) file for details.
