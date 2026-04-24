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
        // 如果已有任意顶层窗口，直接展示并激活它
        let existing = app
            .active_window()
            .or_else(|| app.windows().first().cloned());

        if let Some(window) = existing {
            window.set_visible(true);
            window.present();
            return;
        }
        build_ui(app, Arc::clone(&config));
    });

    app.run();
    Ok(())
}