# Ask & Aura

A command-line and desktop tool for single-turn questioning of LLMs.

![banner](assets/banner.png)

[中文](README.md) | English

## Positioning

In daily use, we often encounter small questions we can't remember at the moment, such as the return value of `strcmp` or how to quickly decompress a tar.gz file. At such times, instead of opening a browser and then ask LLMs, it's more efficient and elegant to use this tool to get answers quickly, saving time and effort.


![banner](assets/ask.png)
- **Ask**: Command-line tool with pipe input support.

![banner](assets/aura.gif)
- **Aura**: GTK4-based desktop GUI with streaming rendering.

## Features

- 🚀 **Fast**: Written in Rust, fast startup and response.
- 🛠 **Configurable**: Supports custom API key, model, API endpoint, etc.
- 🖥 **Lightweight GUI**: Native desktop experience, easy to operate.

## Usage

### Command Line (Ask)

```bash
# Single question
ask -m "How to reverse a list in Python?"

# Simple conversation (some special shell characters may not work)
ask how to unzip a targz file with tar

# Pipe input (when there is a pipe, -m must be used to add the message)
cat src/main.rs | ask -m "Explain what this code does"

# Custom system prompt
ask --prompt "You are a poet" "Write a poem about Rust"
```

It is recommended to use `glow` to beautify the output.

### Desktop GUI (Aura)

- Type a question in the input box, press `Enter` to get a streaming answer.
- Press `↑` / `↓` to navigate through question history.
- Press `Esc` to exit; the window automatically hides when it loses focus.
- Supports basic Markdown rendering.

History is only stored in memory and cleared when the program exits. **Each query is an independent request**; no context is retained across sessions.

## Quick Start

### Manual Compilation

Dependencies: Rust, GTK4 (install `libgtk-4-dev` on Debian/Ubuntu, `gtk4` on Arch), OpenSSL (`libssl-dev`)

```bash
git clone https://github.com/episvr/ask.git
cd ask
make install
```

The tools will be installed to `~/.local/bin`, and a desktop shortcut will be added. Make sure `~/.local/bin` is in your `PATH`. You can also modify the Makefile as needed.

## Configuration

Configuration file path: `~/.config/ask/config.toml`

`system_prompt` is an optional configuration parameter; you can freely define the model's response style.

```toml
api_key = "sk-..."
model = "gpt-4o"
api_base = "https://your-custom-endpoint.com/v1"

system_prompt = "your amazing prompt here"
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss.

## License

[MIT](LICENSE)