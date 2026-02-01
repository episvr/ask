use anyhow::{anyhow, Context, Result};
use clap::Parser;
use is_terminal::IsTerminal;
use std::io::{self, Read};

use ask_core::config::{load_config, DEFAULT_SYSTEM_PROMPT};
use ask_core::api::query_gpt;

// --- CLI Structure ---

#[derive(Parser, Debug)]
#[command(name = "ask", version, about = "A minimal CLI tool to query GPT from the shell.")]
struct Cli {
    /// Explicit message, required in quotes if stdin has content
    #[arg(short, long)]
    message: Option<String>,

    /// Override the default system prompt (one-off)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Free arguments (text input without quotes)
    #[arg(trailing_var_arg = true)]
    text: Vec<String>,
}

// --- Main Logic ---

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Parse CLI arguments
    let cli = Cli::parse();

    // 2. Load configuration (Env > Config File)
    let config = load_config().context("Failed to load configuration")?;

    // 3. Process input source (Stdin vs Args vs -m)
    let input_content = handle_input(&cli).context("Input processing failed")?;

    if input_content.trim().is_empty() {
        return Err(anyhow!("No input provided. Use 'ask --help' for usage."));
    }

    // 4. Determine System Prompt
    let system_prompt = cli.prompt.as_deref().unwrap_or(DEFAULT_SYSTEM_PROMPT);

    // 5. Call API
    let answer = query_gpt(&config, system_prompt, &input_content).await?;

    // 6. Output result
    println!("{}", answer);

    Ok(())
}

fn handle_input(cli: &Cli) -> Result<String> {
    let stdin_has_content = !io::stdin().is_terminal();
    let has_free_args = !cli.text.is_empty();
    let has_m_flag = cli.message.is_some();

    match (stdin_has_content, has_free_args, has_m_flag) {
        // 1. Stdin empty, free args provided -> Use free args
        (false, true, _) => Ok(cli.text.join(" ")),

        // 2. Stdin empty, no free args, -m provided -> Use -m
        (false, false, true) => Ok(cli.message.clone().unwrap()),

        // 3. Stdin has content, no free args, no -m -> Use Stdin
        (true, false, false) => read_stdin(),

        // 4. Stdin has content, free args provided -> Error
        (true, true, _) => Err(anyhow!("stdin detected; use -m with quotes for additional context")),

        // 5. Stdin has content, no free args, -m provided -> Combine Stdin + -m
        (true, false, true) => {
            let context = read_stdin()?;
            let question = cli.message.as_ref().unwrap();
            Ok(format!("[Context]\n{}\n\n[Question]\n{}", context, question))
        }

        // 6. No input provided
        (false, false, false) => Ok(String::new()),
    }
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).context("Failed to read from stdin")?;
    Ok(buffer)
}
