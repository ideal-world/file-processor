#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{clone, env, sync::Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tardis::{basic::result::TardisResult, log::info, tokio, TardisFuns};
mod tauri;
mod uploader;

pub static PARAMS: Lazy<Mutex<FileProcessParams>> = Lazy::new(|| {
    Mutex::new(FileProcessParams {
        upload_mode: true,
        upload_to: None,
        create_folder_url: None,
        upload_file_url: None,
    })
});

#[tokio::main]
async fn main() -> TardisResult<()> {
    env::set_var("RUST_LOG", "debug");

    tauri::build();

    TardisFuns::init(Some("config")).await?;

    info!("started program.");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileProcessParams {
    upload_mode: bool,
    upload_to: Option<String>,
    create_folder_url: Option<String>,
    upload_file_url: Option<String>,
}
