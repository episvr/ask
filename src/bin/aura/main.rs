mod ui;
mod markdown;

use anyhow::Result;
use gtk4 as gtk;
use gtk::{prelude::*, Application};
use std::sync::Arc;
use ask_core::config::load_config;
use ui::build_ui;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Aura v{}", env!("CARGO_PKG_VERSION"));
    let app = Application::builder()
        .application_id("com.example.aura")
        .build();

    let config = load_config()?;

    app.connect_activate(move |app| {
        // 如果窗口已存在，直接展示（单例模式）
        if let Some(window) = app.active_window() {
            window.set_visible(true);
            window.present();
            return;
        }
        build_ui(app, Arc::clone(&config));
    });

    app.run();
    Ok(())
}