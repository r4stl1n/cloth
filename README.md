# Nexus CLI

A command-line interface tool built in Rust for processing LLM patterns through the OpenWebUI API.

## Overview

Nexus CLI provides a streamlined way to interact with Large Language Models using predefined patterns. It integrates seamlessly with OpenWebUI to process text through various AI models with structured prompts.

## Features

- 🎯 **Pattern-based Processing**: Use predefined patterns 
- 🤖 **Agentic Workflows**: Deploy intelligent agent teams for complex tasks
- 🔌 **OpenWebUI Integration**: Direct integration with OpenWebUI API
- ⚙️ **Flexible Configuration**: Easy setup and configuration management
- 🚀 **Raw Query Support**: Process custom prompts alongside patterns

## Installation

### Prerequisites

- Rust 1.87.0 or later
- Cargo package manager
- Access to an OpenWebUI instance

### Build from Source

```bash
git clone https://github.com/r4stl1n/nexus
cd nexus
cargo build --release
##The compiled binary will be available at `target/release/nexus`.
```
To install do the following
``` bash
cargo install --path ./
```
## Quick Start
1. **Setup Configuration**
``` bash
   nexus config setup
```
1. **List Available Models**
``` bash
   nexus owui list-models
```
1. **Process Text with a Pattern**
``` bash
   nexus pattern process --name extract_wisdom --model your-model --query "Your text here"
```

## Usage Examples

### Basic Pattern Processing
```bash
# Process text using a pattern
nexus pattern process --name summarize --model model-name --query "Text to summarize"
```

### Agentic Team Processing
``` bash
# Run a security analysis task
nexus agentic process --team security --task "Review the authentication system for potential vulnerabilities in architecture.md"

# Development team task
nexus agentic process --team development --task "Design a scalable microservices architecture design and write to design.md"

# Generic problem-solving task
nexus agentic process --team generic --task "Write a short blog post about microservices"
```

### Using with System Clipboard (macOS)
``` bash
# Process clipboard content with a pattern
pbpaste | nexus pattern process --name extract_wisdom

# Extract wisdom from clipboard content
pbpaste | nexus pattern process --name extract_wisdom --model your-model
```
### Chaining Nexus Commands
``` bash
# Chain patterns: first summarize, then create a one-sentence summary
cat test.md | nexus pattern process --name summarize | nexus pattern raw --prompt "Return a one sentence summary of the text provided."

# Process file through multiple stages
cat document.txt | nexus pattern process --name humanize | nexus pattern process --name summarize

# Extract wisdom and then summarize the results
pbpaste | nexus pattern process --name extract_wisdom | nexus pattern raw --prompt "Create a bullet-point list of the key insights"
```
### Pipeline Processing
``` bash
# Complex pipeline: clipboard → extract wisdom → summarize → final formatting
pbpaste | nexus pattern process --name extract_wisdom --model gpt-4 | nexus pattern process --name summarize --model gpt-3.5 | nexus pattern raw --prompt "Format this as a tweet-length summary"

# File processing pipeline
cat research_paper.txt | nexus pattern process --name extract_wisdom | nexus pattern raw --prompt "Convert these insights into actionable recommendations"
```

### Working with Models
``` bash
# List all available models
nexus owui list-models

# Send a direct completion request
nexus owui completion --model model-name --query "Your prompt"
```
### Pattern Operations
``` bash
# List all available patterns
nexus pattern list

# View a specific pattern
nexus pattern view --pattern extract_wisdom

# Process text using a pattern
nexus pattern process --name summarize --model model-name --query "Text to summarize"

# Process with custom pattern directory
nexus pattern --pattern-directory /path/to/patterns list
```
### Raw Query Processing
``` bash
# Process a raw prompt
nexus pattern raw --model model-name --prompt "Your custom prompt" --query "Input text"
```
## Pattern System
Nexus uses a directory-based pattern system. Each pattern is defined in a separate folder containing a `pattern.md` file.
### Default Pattern Structure
``` 
patterns/
├── extract_wisdom/
│   └── pattern.md
├── humanize/
│   └── pattern.md
└── summarize/
│   └── pattern.md
```
### Creating Custom Patterns
1. Create a new directory in your patterns folder
2. Add a `pattern.md` file with your prompt template
3. Use the pattern with `nexus pattern process --name your-pattern-name`

## Command Reference
Click to expand full command reference### `nexus`
Main application entry point.
**Usage:** `nexus <COMMAND>`
**Subcommands:**
- `config` — Configuration management
- `owui` — OpenWebUI operations
- `pattern` — Pattern operations

### `nexus config`
**Subcommands:**
- `view` — Display current configuration
- `setup` — Initialize configuration

### `nexus owui`
**Subcommands:**
- `list-models` — List available models
- `completion`  — Send completion request

### `nexus pattern`
**Options:**
- `--pattern-directory <PATH>` — Custom pattern directory

**Subcommands:**
- `list` — List all patterns
- `view --pattern <NAME>` — View pattern details
- `process --name <NAME> [--model <MODEL>] [--query <QUERY>]` — Process with pattern
- `raw --prompt <PROMPT> [--model <MODEL>] [--query <QUERY>]` — Process raw prompt

## Development
### Project Structure
details
summary
summary
details
``` 
src/
├── commands/          # CLI command implementations
├── integrations/      # OpenWebUI and external service integrations
├── managers/          # Core business logic and state management
├── utils/            # Shared utility functions
└── main.rs           # Application entry point
``` 

## Troubleshooting
### Common Issues
- **Connection errors**: Verify your OpenWebUI instance is running and accessible
- **Model not found**: Use `nexus owui list-models` to see available models
- **Pattern not found**: Check pattern directory path and file structure
- **Permission errors**: Ensure the binary has execution permissions

### Logging
Enable detailed logging for debugging:
``` bash
RUST_LOG=debug nexus pattern list
```
## Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License
See [LICENSE](LICENSE) file for details.
