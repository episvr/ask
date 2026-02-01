# Ask & Aura

A lightweight tool for querying large language models (LLM/GPT) locally.

- **Ask**: A command-line tool (CLI) for quick queries and piped input.
- **Aura**: A lightweight desktop GUI based on GTK4, suitable for interactive conversations.

> **中文版请见**：[README.md](README.md)

## Features

- 🚀 **Fast**: Developed in Rust for quick startup and response times.
- 🛠 **Configurable**: Supports custom API keys, models, and API endpoints via configuration files.
- 🖥 **Lightweight GUI**: `aura` is based on GTK4, providing a native desktop experience with streaming answer rendering.

## Design Positioning

`Ask` and `Aura` are not designed as "agents" for multi-step automation. They are better suited as tools for single-turn conversations, such as quick reference-style Q&A, aiming to save the time of opening a browser to search. The focus is on speed, intuitiveness, and easy integration into pipelines and scripts.

## Prerequisites

- Rust (latest stable version)
- GTK4 development files (`libgtk-4-dev` on Debian/Ubuntu, `gtk4` on Arch)
- OpenSSL (`libssl-dev`)

## Installation

### From Source

```bash
git clone https://github.com/episvr/ask.git
cd ask
make install
```

This command compiles and installs `ask` and `aura` to `~/.local/bin`, and adds shortcuts to the desktop. Ensure that `~/.local/bin` is in your `PATH`.

## Configuration

Settings are read from a configuration file (`~/.config/ask/config.toml`).

Set up `~/.config/ask/config.toml` as follows:

```toml
api_key = "sk-..."
model = "gpt-4o"
api_base = "https://your-custom-endpoint.com/v1"
```

## Usage

### CLI (Ask)

Supports multiple query methods:

- **Single-shot queries**:

```bash
ask -m "How do I reverse a list in Python?"
```

You can also omit quotes and the "-m" parameter for simple queries, but this method only works when no additional parameters are provided:

```bash
ask how to unzip a tar.gz file with tar?
```

- **Piped input**:

When using piped input, be sure to include the "-m" parameter:

```bash
cat src/main.rs | ask -m "Explain what this code does"
```

- **Interactive / Custom system prompts**:

```bash
ask --prompt "You are a poetic assistant" "Write a poem about Rust"
```

For a better reading experience, it is recommended to use the output with the `glow` tool.

### GUI (Aura)

Launch from the terminal or application menu:

```bash
aura
```

Interaction notes:

- **Quick questions**: Type your question in the input box and press `Enter` to get streaming answers.
- **History navigation**: Use `↑` / `↓` to navigate through previous questions in the input box.
- **Spotlight mode**: Press `Esc` to exit the application; the window hides automatically when it loses focus.
- **Markdown support**: Answers support basic Markdown rendering.

The current implementation uses in-memory caching (cleared when the program exits).

Note: This tool **does not maintain contextual memory across sessions**. Each query is sent as an independent request to the model; history is only used during the current session for input navigation and local cache lookups to avoid repeated API calls for identical queries. It does not automatically accumulate or append context to subsequent requests across sessions.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss the proposed implementation.

## License

[MIT](LICENSE)
