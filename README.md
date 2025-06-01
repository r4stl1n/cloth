# Cloth CLI

A command-line interface tool built in Rust for processing LLM patterns through the OpenWebUI API.

## Overview

Cloth CLI provides a streamlined way to interact with Large Language Models using predefined patterns. It integrates seamlessly with OpenWebUI to process text through various AI models with structured prompts.

## Features

- 🎯 **Pattern-based Processing**: Use predefined patterns 
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
git clone https://github.com/r4stl1n/cloth
cd cloth
cargo build --release
```
```
The compiled binary will be available at `target/release/cloth`.
### Add to PATH (Optional)
``` bash
# Copy to a directory in your PATH
cp target/release/cloth /usr/local/bin/
```
## Quick Start
1. **Setup Configuration**
``` bash
   cloth config setup
```
1. **List Available Models**
``` bash
   cloth owui list-models
```
1. **Process Text with a Pattern**
``` bash
   cloth pattern process --name extract_wisdom --model your-model --query "Your text here"
```

## Usage Examples

### Basic Pattern Processing
```bash
# Process text using a pattern
cloth pattern process --name summarize --model model-name --query "Text to summarize"
```
### Using with System Clipboard (macOS)
``` bash
# Process clipboard content with a pattern
pbpaste | cloth pattern process --name extract_wisdom

# Extract wisdom from clipboard content
pbpaste | cloth pattern process --name extract_wisdom --model your-model
```
### Chaining Cloth Commands
``` bash
# Chain patterns: first summarize, then create a one-sentence summary
cat test.md | cloth pattern process --name summarize | cloth pattern raw --prompt "Return a one sentence summary of the text provided."

# Process file through multiple stages
cat document.txt | cloth pattern process --name humanize | cloth pattern process --name summarize

# Extract wisdom and then summarize the results
pbpaste | cloth pattern process --name extract_wisdom | cloth pattern raw --prompt "Create a bullet-point list of the key insights"
```
### Pipeline Processing
``` bash
# Complex pipeline: clipboard → extract wisdom → summarize → final formatting
pbpaste | cloth pattern process --name extract_wisdom --model gpt-4 | cloth pattern process --name summarize --model gpt-3.5 | cloth pattern raw --prompt "Format this as a tweet-length summary"

# File processing pipeline
cat research_paper.txt | cloth pattern process --name extract_wisdom | cloth pattern raw --prompt "Convert these insights into actionable recommendations"
```

### Working with Models
``` bash
# List all available models
cloth owui list-models

# Send a direct completion request
cloth owui completion --model model-name --query "Your prompt"
```
### Pattern Operations
``` bash
# List all available patterns
cloth pattern list

# View a specific pattern
cloth pattern view --pattern extract_wisdom

# Process text using a pattern
cloth pattern process --name summarize --model model-name --query "Text to summarize"

# Process with custom pattern directory
cloth pattern --pattern-directory /path/to/patterns list
```
### Raw Query Processing
``` bash
# Process a raw prompt
cloth pattern raw --model model-name --prompt "Your custom prompt" --query "Input text"
```
## Pattern System
Cloth uses a directory-based pattern system. Each pattern is defined in a separate folder containing a `pattern.md` file.
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
3. Use the pattern with `cloth pattern process --name your-pattern-name`

## Command Reference
Click to expand full command reference### `cloth`
Main application entry point.
**Usage:** `cloth <COMMAND>`
**Subcommands:**
- `config` — Configuration management
- `owui` — OpenWebUI operations
- `pattern` — Pattern operations

### `cloth config`
**Subcommands:**
- `view` — Display current configuration
- `setup` — Initialize configuration

### `cloth owui`
**Subcommands:**
- `list-models` — List available models
- `completion`  — Send completion request

### `cloth pattern`
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
- **Model not found**: Use `cloth owui list-models` to see available models
- **Pattern not found**: Check pattern directory path and file structure
- **Permission errors**: Ensure the binary has execution permissions

### Logging
Enable detailed logging for debugging:
``` bash
RUST_LOG=debug cloth pattern list
```
## Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License
See [LICENSE](LICENSE) file for details.
