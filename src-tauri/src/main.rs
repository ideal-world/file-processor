#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use serde::{Deserialize, Serialize};
use tardis::{basic::result::TardisResult, log::info, tokio, TardisFuns};
mod tauri;
mod uploader;

#[tokio::main]
async fn main() -> TardisResult<()> {
    env::set_var("RUST_LOG", "debug");

    tauri::build();

    TardisFuns::init(Some("config")).await?;
    
    info!("started program.");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    create_folder_url: String,
    upload_file_url: String,
}
