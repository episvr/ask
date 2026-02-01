# Ask & Aura

一个轻量且注重隐私的 Rust 工具集，用于在本地向大语言模型（LLM/GPT）提问。

- **Ask**：命令行工具（CLI），适合快速提问与管道化输入。
- **Aura**：基于 GTK4 的轻量桌面 GUI，适合交互式对话。

> 默认文档语言为中文。英文版请见：[README.en.md](README.en.md)。

## 特点

- 🚀 **速度快**：使用 Rust 开发，启动与响应快速。
- 🛠 **易于配置**：支持自定义模型、API 基址和系统提示。
- 🖥 **原生 GUI**：`aura` 基于 GTK4，提供原生 Linux 桌面体验。

## 定位

`Ask` 和 `Aura` 并非用于代替能执行多步自动化的 CLI “agent”。它们更侧重于提供简洁、直接的回答，例如 TL;DR 摘要、要点式解释、或备忘式（cheat‑sheet）提示。工具优先考虑快速响应与隐私保护，同时便于在管道与脚本中使用，而不是复杂自动化。

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

该命令会编译并将 `ask` 与 `aura` 安装到 `~/.local/bin`，并为 `aura` 安装桌面启动器。请确保 `~/.local/bin` 在你的 `PATH` 中。

## 配置

你可以通过环境变量或配置文件来配置工具：

### 环境变量

在你的 `.bashrc` / `.zshrc` 或 `.env` 中设置：

```bash
export ASK_API_KEY="sk-..."
export ASK_MODEL="gpt-4o-mini"  # 可选，默认: gpt-4o-mini
export ASK_API_BASE="https://api.openai.com/v1"  # 可选
```

### 配置文件

在 `~/.config/ask/config.toml` 中设置：

```toml
api_key = "sk-..."
model = "gpt-4o"
api_base = "https://your-custom-endpoint.com/v1"
```

## 使用

### CLI（Ask）

`Ask` 支持多种提问方式，按需选择：

- **参数模式（单次）**：
	```bash
	ask "How do I reverse a list in Python?"
	```
	将问题作为命令参数传入，适合简短的问题与一次性查询。

	> [!Note]
    > 对于非常简单的单行输入可以省略引号：
	> ```bash
	> ask How do I reverse a list in Python
	> ```
	> 但不带引号的形式在遇到特殊字符、Shell 扩展或多行输入时可能不可靠。遇到这类情况请使用带引号的形式（`ask "..."`）或通过管道传入（`... | ask`）。

- **管道输入**：
	```bash
	cat src/main.rs | ask "Explain what this code does"
	```
	将文件或命令输出通过 stdin 传入，适合代码解释、文件摘要等场景。

- **交互 / 自定义系统提示**：
	```bash
	ask --prompt "You are a poetic assistant" "Write a poem about Rust"
	```
	可设置自定义系统提示；若省略最后的问题参数，会进入交互模式并读取 stdin。

- **脚本化 / 批量**：在脚本或流水线中使用 `ask` 以生成简洁的摘要、TL;DR 或备忘片段。

### GUI（Aura）

在终端或应用启动器中运行：

```bash
aura
```

`Aura` 提供类似 Spotlight/Raycast 的极简交互体验：
- **快速提问**：输入问题并回车（Enter）即可获取流式回答。
- **历史记录**：使用方向键 `↑` / `↓` 在输入框中快速切换历史提问。
- **Spotlight 模式**：
    - `Esc`：**退出**程序。
    - **失去焦点**：窗口会自动**隐藏**，以便你快速回到工作流。再次启动应用即可瞬间恢复。
- **Markdown 支持**：回答内容支持基础的 Markdown 格式化渲染。

## 参与贡献

欢迎提交 PR。对于重大变更，请先打开 Issue 讨论方案。

## 许可证

[MIT](LICENSE)
