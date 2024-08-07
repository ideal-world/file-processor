#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, sync::Mutex};
#[cfg(debug_assertions)]
use tardis::config::config_dto::TardisConfig;
#[cfg(debug_assertions)]
use tardis::TardisFuns;
use tardis::{basic::result::TardisResult, tokio};
mod tauri;
mod uploader;

pub static PARAMS: Lazy<Mutex<FileProcessParams>> = Lazy::new(|| {
    Mutex::new(FileProcessParams {
        title: "".to_string(),
        upload: None,
    })
});

fn get_params() -> FileProcessParams {
    (*PARAMS.lock().unwrap()).clone()
}

#[tokio::main]
async fn main() -> TardisResult<()> {
    env::set_var("RUST_LOG", "debug");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let raw_params = args[1].as_str();
        match reqwest::Url::parse(raw_params) {
            Ok(url) => {
                let params = tauri::parse_params(&url);
                info!("params: {:?}", params);
                let mut params_set = PARAMS.lock().unwrap();
                *params_set = params;
            }
            Err(_) => log::error!("parse url fail!:{raw_params}"),
        }
    } else {
        // mock
        let mut params_set = PARAMS.lock().unwrap();
        *params_set = FileProcessParams {
            title: "请按使用文档调用（以下为示例）".to_string(),
            upload: Some(FileUploadProcessParams {
                target_kind_key: "".to_string(),
                target_obj_key: "".to_string(),
                overwrite: false,
                upload_metadata_url: "".to_string(),
                upload_metadata_rename_filed: None,
                upload_fixed_metadata: None,
                upload_fixed_headers: None,
            }),
        };
    }

    // Debug时使用此初始化 ``src-tauri/config``
    #[cfg(debug_assertions)]
    {
        let config = TardisConfig::init(Some("src-tauri/config")).await?;
        TardisFuns::init_conf(config).await?;
    }

    tauri::build();

    Ok(())
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileProcessParams {
    pub title: String,
    pub upload: Option<FileUploadProcessParams>,
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileUploadProcessParams {
    pub target_kind_key: String,
    pub target_obj_key: String,
    pub overwrite: bool,
    // must be post
    pub upload_metadata_url: String,
    pub upload_metadata_rename_filed: Option<uploader::UploadMapFiled>,
    // fixed upload filed
    pub upload_fixed_metadata: Option<HashMap<String, Value>>,
    // fixed upload headers
    pub upload_fixed_headers: Option<HashMap<String, String>>,
}
