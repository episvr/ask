# Ask & Aura

一个轻量的用于在本地向大语言模型（LLM/GPT）提问的工具。

- **Ask**：命令行工具（CLI），适合快速提问与管道化输入。
- **Aura**：基于 GTK4 的轻量桌面 GUI，适合交互式对话。

> **English version available**: See [README.en.md](README.en.md) for the English documentation.

## 特性

- 🚀 **快速**：使用 Rust 开发，启动与响应迅速；
- 🛠 **可配置**：支持通过配置文件自定义 API Key、模型与 API 基址；
- 🖥 **轻量 GUI**：`aura` 基于 GTK4，提供本地原生桌面体验，并支持流式渲染答案；

## 设计定位

在日常生活中，总是会遇到一些一时间记不住的小问题，比如："strcmp"的返回值是正数、零还是负数的意思，或者如何快速解压一个 tar.gz 文件。这时候打开浏览器搜索可能会浪费时间，而使用 `Ask` 可以快速得到答案，节省精力。

`Ask` 与 `Aura` 并非为执行多步自动化的“agent”而设计；它们更适合作为单次对话工具，例如快速查阅式问答。重点在于快速、直观，并易于集成到管道中。

## 先决条件

- Rust（最新稳定版）
- GTK4 开发文件（Debian/Ubuntu：`libgtk-4-dev`，Arch：`gtk4`）
- OpenSSL（`libssl-dev`）

## 安装

### 从源码

```bash
git clone https://github.com/episvr/ask.git
cd ask
make install
```

该命令会编译并将 `ask` 与 `aura` 安装到 `~/.local/bin`，并将快捷方式安装到桌面。请确保 `~/.local/bin` 在你的 `PATH` 中。

## 配置

从配置文件读取设置（`~/.config/ask/config.toml`）。

在 `~/.config/ask/config.toml` 中设置：

```toml
api_key = "sk-..."
model = "gpt-4o"
api_base = "https://your-custom-endpoint.com/v1"
```

## 使用

### CLI（Ask）

支持多种提问方式：

- **单次参数**：

```bash
ask -m "How do I reverse a list in Python?"
```

你也可以省略引号和 "-m" 参数直接输入，这种方式适用于简单对话，但仅在未附加其他参数时有效：

```bash
ask how to unzip a tar.gz file with tar?
```

- **管道输入**：

通过管道输入时，请务必附加 "-m" 参数：

```bash
cat src/main.rs | ask -m "Explain what this code does"
```

- **交互 / 自定义系统提示**：

```bash
ask --prompt "You are a poetic assistant" "Write a poem about Rust"
```

为了更好的阅读体验，建议将输出与 `glow` 工具结合使用。

### GUI（Aura）

从终端或应用菜单启动：

```bash
aura
```

交互说明：

- **快速提问**：在输入框中输入问题并按 `Enter` 获取流式回答；
- **历史导航**：使用 `↑` / `↓` 在输入框内切换历史问题；
- **Spotlight 模式**：按 `Esc` 退出应用；窗口失去焦点时自动隐藏；
- **Markdown 支持**：回答文本支持基础 Markdown 渲染。

当前实现采用内存缓存（程序退出后清空）。

注意：该工具**不具备跨会话的上下文记忆**。每次提问都会作为独立请求发送给模型；历史记录只在当前程序运行期间用于输入框的历史导航和本地缓存查重（以避免对相同问题重复调用 API），不会在不同会话之间自动累积或作为上下文附加到后续请求中。

## 参与贡献

欢迎提交 PR。对于重大变更，请先打开 Issue 讨论实现方案。

## 许可证

[MIT](LICENSE)
