#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use tardis::{basic::result::TardisResult, log::info, tokio, TardisFuns};
mod ui;
mod uploader;

#[tokio::main]
async fn main() -> TardisResult<()> {
    env::set_var("RUST_LOG", "debug");

    TardisFuns::init(Some("config")).await?;

    ui::build();

    info!("started program.");

    Ok(())
}
