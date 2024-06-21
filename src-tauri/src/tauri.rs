use crate::uploader::{self, UploadProgressResp};
use tardis::{basic::result::TardisResult, log::info};
use tauri::http::{self};
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
async fn upload_files(files_uri: &str) -> TardisResult<bool> {
    info!("upload_files: {}", files_uri);
    uploader::upload_files(files_uri).await
}

#[tauri::command]
async fn get_progress() -> TardisResult<UploadProgressResp> {
    uploader::get_progress().await
}

pub fn build() {
    tauri::Builder::default()
        // .register_uri_scheme_protocol("file-processor", move |_app, request| {
        //     info!("-----------------");
        //     if let Ok(data) = std::fs::read(&request.uri().path()[1..]) {
        //         http::Response::builder().body(data).unwrap()
        //     } else {
        //         http::Response::builder()
        //             .status(http::StatusCode::BAD_REQUEST)
        //             .body("failed to read file".as_bytes().to_vec())
        //             .unwrap()
        //     }
        // })
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            app.listen("deep-link://new-url", |url| {
                info!("----{}", url.payload());
                dbg!(url);
            });
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .clear_targets()
                .targets([
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Stderr),
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("./file-processor".into()),
                    }),
                ])
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![upload_files, get_progress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
