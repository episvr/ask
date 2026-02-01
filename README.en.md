# Ask & Aura

A minimal tool for querying LLMs (GPT) from your local environment.

- **Ask**: A command-line interface (CLI) for quick questions and piping input.
- **Aura**: A lightweight GTK4 GUI for a more interactive chat experience.

## Features

- 🚀 **Fast**: Built with Rust.
- 🛠 **Configurable**: Supports custom models, API bases, and system prompts.
- 🖥 **GTK4 GUI**: Native Linux feel with `aura`.

## Design & Positioning

Ask & Aura are intentionally not designed as autonomous CLI "agents" that execute multi-step workflows. Instead, they're focused on short, focused interactions — TL;DR summaries, concise explanations, and cheat‑sheet style answers. They prioritize quick, privacy-first responses and easy integration into pipes and scripts rather than complex automation.

## Prerequisites

- Rust (latest stable)
- GTK4 development files (`libgtk-4-dev` on Debian/Ubuntu, `gtk4` on Arch)
- OpenSSL (`libssl-dev`)

## Installation

### From Source

```bash
git clone https://github.com/episvr/ask.git
cd ask
make install
```

This will compile the project and install `ask` and `aura` to `~/.local/bin`, and install desktop entries for `aura`.

Ensure `~/.local/bin` is in your `PATH`.

## Configuration

You can configure the tool using either environment variables or a config file.

### 1. Environment Variables

Store these in your `.bashrc` / `.zshrc` or a `.env` file (if running locally):

```bash
export ASK_API_KEY="sk-..."
export ASK_MODEL="gpt-4o-mini" # Optional, default: gpt-4o-mini
export ASK_API_BASE="https://api.openai.com/v1" # Optional
```

### 2. Config File

Create a file at `~/.config/ask/config.toml`:

```toml
api_key = "sk-..."
model = "gpt-4o"
api_base = "https://your-custom-endpoint.com/v1"
```

## Usage

### CLI (Ask)

Ask supports multiple ways to ask a question — pick the mode that fits your workflow:

- **Single-shot argument**: `ask "How do I reverse a list in Python?"` — pass a quick question directly as an argument.

> [!Note]
> You can also run `ask How do I reverse a list in Python` without quotes for very simple, single-line inputs; however, this mode may not handle special characters, shell expansion, or multi-line input reliably — in those cases prefer quoting (`ask "..."`) or piping (`... | ask`).  

- **Piped input**: `cat src/main.rs | ask "Explain what this code does"` — send file or command output on stdin to be summarized or explained.
- **Interactive / prompt-driven**: `ask --prompt "You are a poetic assistant" "Write a poem about Rust"` — set a custom system prompt; omit the final argument to enter an interactive session that reads from stdin.
- **Scriptable & batch usage**: Use `ask` in scripts or pipelines to generate concise summaries, TL;DRs, or cheat‑sheet snippets.

Examples:

**Quick question:**
```bash
ask "How do I reverse a list in Python?"
```

**Pipe input:**
```bash
cat src/main.rs | ask "Explain what this code does"
```

**Interactive mode (via stdin):**
```bash
ask --prompt "You are a poetic assistant" "Write a poem about Rust"
```

### GUI (Aura)

Simply launch `aura` from your terminal or application launcher.

`Aura` provides a minimalist, Spotlight/Raycast-like experience:
- **Quick Query**: Type your question and press `Enter` to get streaming answers.
- **History Navigation**: Use `↑` / `↓` arrow keys in the input bar to quickly cycle through previous questions.
- **Spotlight Mode**:
    - `Esc`: **Quit** the application.
    - **Focus-out**: The window automatically **hides** when it loses focus, allowing you to return to your work immediately. Launching it again restores it instantly.
- **Markdown Support**: Responses are rendered with basic Markdown support.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](LICENSE)
