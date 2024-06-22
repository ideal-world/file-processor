use crate::{
    uploader::{self, UploadStatsResp},
    FileProcessParams, PARAMS,
};
use log::info;
use tardis::basic::result::TardisResult;
use tauri::Window;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
async fn upload_files(files_uri: &str, window: Window) -> TardisResult<UploadStatsResp> {
    info!("upload_files: {}", files_uri);
    uploader::upload_files(files_uri, window).await
}

#[tauri::command]
async fn get_params() -> TardisResult<FileProcessParams> {
    Ok((*PARAMS.lock().unwrap()).clone())
}

pub fn build() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![upload_files, get_params])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
