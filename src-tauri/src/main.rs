#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{clone, env, sync::Mutex};
use tardis::{basic::result::TardisResult, tokio, TardisFuns};
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
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        let mut raw_params = args[1].as_str();
        if raw_params.contains("//") {
            let index = raw_params.find("//").unwrap();
            raw_params = &raw_params[index + 2..];
        }
        if raw_params.ends_with("/") {
            raw_params = &raw_params[..raw_params.len() - 1];
        }
        let params = TardisFuns::json
            .str_to_obj::<FileProcessParams>(
                &TardisFuns::crypto
                    .base64
                    .decode_to_string(raw_params)
                    .unwrap(),
            )
            .unwrap();
        info!("params: {:?}", params);
        let mut params_set = PARAMS.lock().unwrap();
        *params_set = params;
    }

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
