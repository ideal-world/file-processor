use crate::uploader::{self, UploadProgressResp};
use tardis::basic::result::TardisResult;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
async fn upload_files(files_uri: &str) -> TardisResult<bool> {
    uploader::upload_files(files_uri).await
}

#[tauri::command]
async fn get_progress() -> TardisResult<UploadProgressResp> {
    uploader::get_progress().await
}

pub fn build() {
    tauri::Builder::default()
        //  .plugin(
        // tauri_plugin_log::Builder::default()
        //     .clear_targets()
        //     .targets([
        //         Target::new(TargetKind::Webview),
        //         Target::new(TargetKind::Stderr),
        //         Target::new(TargetKind::Stdout),
        //         Target::new(TargetKind::LogDir {
        //             file_name: Some("./report".into()),
        //         }),
        //     ])
        //     .build(),
        // )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![upload_files, get_progress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
