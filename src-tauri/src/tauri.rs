use std::{collections::HashMap, env};

use crate::{
    uploader::{self, UploadStatsResp}, FileProcessParams, FileUploadProcessParams, PARAMS
};
use base64::{engine::general_purpose, Engine as _};
use log::info;
use tardis::{
    basic::result::TardisResult, config::config_dto::TardisConfig, futures::executor, TardisFuns,
};
use tauri::{path::BaseDirectory, Manager, Window};
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
async fn upload_files(files_uris: Vec<String>, window: Window) -> TardisResult<UploadStatsResp> {
    info!("upload_files: {:?}", files_uris);
    uploader::upload_files(files_uris, window).await
}

#[tauri::command]
async fn get_params() -> TardisResult<FileProcessParams> {
    Ok((*PARAMS.lock().unwrap()).clone())
}

fn set_params(params: FileProcessParams) -> TardisResult<()> {
    let mut params_set = PARAMS.lock().unwrap();
    *params_set = params;
    Ok(())
}

pub fn build() {
    tauri::Builder::default()
        .setup(|app| {
            //macos use this way to init config
            #[cfg(target_os = "macos")]
            {
                let config_path = app
                    .path()
                    .resolve("config", BaseDirectory::Resource)
                    .expect("get resource path err!");
                executor::block_on(async {
                    let config_path = config_path.strip_prefix("/").unwrap();
                    let config = TardisConfig::init(Some(config_path.to_str().unwrap()))
                        .await
                        .expect("can't find config");
                    info!("====config:{config:?}");
                    TardisFuns::init_conf(config)
                        .await
                        .expect("can't init config");
                });
                // app.listen("deep-link://new-url", |url| {
                //     let urls: Vec<tauri::Url> = serde_json::from_str(url.payload()).unwrap();
                //     if let Some(url) = urls.get(0) {
                //         let base64 = TardisCryptoBase64 {};
                //         let params = TardisFuns::json
                //             .str_to_obj::<FileProcessParams>(
                //                 base64
                //                     .decode_to_string(url.host_str().unwrap())
                //                     .unwrap()
                //                     .as_str(),
                //             )
                //             .unwrap();
                //         info!("params: {:?}", params);
                //         let _ = set_params(params);
                //     }
                // });
            }
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app, _event| {
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if let tauri::RunEvent::Opened { urls } = _event {
                // info!("open url============={urls}",);
                if let Some(url) = urls.get(0) {
                    let params = parse_params(url);
                    info!("Opened url parse to params: {:?}", params);
                    let _ = set_params(params);
                }
            }
        });
}

fn parse_params(url: &reqwest::Url) -> FileProcessParams {
    TardisFuns::json
        .str_to_obj::<FileProcessParams>(
            &String::from_utf8(
                general_purpose::URL_SAFE
                    .decode(url.host_str().unwrap())
                    .unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
}

#[test]
fn test_parse_params(){
  let mut upload_fixed_headers=HashMap::new();
  upload_fixed_headers.insert(String::from("Token"), String::from("78hhySDFGT56gGh65"));
  assert_eq!(parse_params(&reqwest::Url::parse("file-processor://eyJ0aXRsZSI6IuS4iuS8oOWIsO-8mmtub3dsZWRnZS03NC8iLCJ1cGxvYWQiOnsidGFyZ2V0X2tpbmRfa2V5IjoiIiwidGFyZ2V0X29ial9rZXkiOiIiLCJvdmVyd3JpdGUiOnRydWUsInVwbG9hZF9tZXRhZGF0YV91cmwiOiJ4eHh4IiwidXBsb2FkX2ZpeGVkX2hlYWRlcnMiOnsiVG9rZW4iOiI3OGhoeVNERkdUNTZnR2g2NSJ9fX0=").unwrap()),FileProcessParams{ title: String::from("上传到：knowledge-74/"), upload: Some(FileUploadProcessParams{ target_kind_key: String::new(), target_obj_key: String::new(), overwrite: true, upload_metadata_url: String::from("xxxx"), upload_metadata_rename_filed: None, upload_fixed_metadata: None, upload_fixed_headers: Some(upload_fixed_headers) }) })
}
