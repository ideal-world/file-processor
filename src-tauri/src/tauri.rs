use crate::{
    uploader::{self, UploadProgressResp},
    FileProcessParams, PARAMS,
};
use tardis::{basic::result::TardisResult, log::info, TardisFuns};
use tauri_plugin_cli::CliExt;

#[tauri::command]
async fn upload_files(files_uri: &str) -> TardisResult<bool> {
    info!("upload_files: {}", files_uri);
    uploader::upload_files(files_uri).await
}

#[tauri::command]
async fn get_progress() -> TardisResult<UploadProgressResp> {
    uploader::get_progress().await
}

#[tauri::command]
async fn get_params() -> TardisResult<FileProcessParams> {
    Ok((*PARAMS.lock().unwrap()).clone())
}

pub fn build() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                Ok(matches) => {
                    let p = match matches.args.get("params") {
                        Some(p) if p.value.as_str().is_some() => {
                            let mut raw_params = p.value.as_str().unwrap();
                            if raw_params.contains("//") {
                                let index = raw_params.find("//").unwrap();
                                raw_params = &raw_params[index + 2..];
                            }
                            if raw_params.ends_with("/") {
                                raw_params = &raw_params[..raw_params.len() - 1];
                            }
                            TardisFuns::json
                                .str_to_obj::<FileProcessParams>(
                                    &TardisFuns::crypto
                                        .base64
                                        .decode_to_string(raw_params)
                                        .unwrap(),
                                )
                                .unwrap()
                        }
                        _ => FileProcessParams {
                            upload_mode: true,
                            upload_to: Some("111".to_string()),
                            create_folder_url: None,
                            upload_file_url: None,
                        },
                    };
                    info!("params: {:?}", p);
                    let mut params = PARAMS.lock().unwrap();
                    *params = p;
                }
                Err(_) => {}
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            upload_files,
            get_progress,
            get_params
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
