use crate::{
    uploader::{self, UploadStatsResp},
    FileProcessParams, PARAMS,
};
use log::info;
use tardis::basic::result::TardisResult;
use tauri::{Manager, Window};
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
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let current_monitor = window.current_monitor().unwrap().unwrap();
            let screen_size = current_monitor.size();
            let window_size = window.outer_size().unwrap();

            let new_x = screen_size.width - window_size.width;
            // 非精确计算任务栏高度，否则需要引用winapi
            let new_y = screen_size.height - 80 - window_size.height;

            window
                .set_position(tauri::Position::Physical((new_x, new_y).into()))
                .unwrap();
            Ok(())
        })
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
