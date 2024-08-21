use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tardis::{
    basic::{error::TardisError, result::TardisResult},
    futures::{lock::Mutex, stream, StreamExt as _},
    rand::random,
    tokio::{
        fs::{read_dir, File},
        io::AsyncReadExt,
        spawn,
        sync::{mpsc, Semaphore},
    },
    web::reqwest,
    TardisFuns,
};
use tauri::{async_runtime::TokioJoinHandle, Emitter as _, Window};

use crate::{processor_config::{ProcessorConfig, DOMAIN_CODE}, FileUploadProcessParams};

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadProgressResp {
    pub uploaded_file_numbers: usize,
    pub uploaded_file_size: u64,
    pub current_files: Vec<UploadFileInfo>,
    pub success_files: Vec<UploadFileInfo>,
    pub fail_files: Vec<UploadFileInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UploadFileInfo {
    pub id: String,
    pub name: String,
    // Relative path
    pub relative_path: PathBuf,
    pub size: u64,
    pub mime_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum UploadFileInfoFiled {
    Name,
    RelativePath,
    Size,
    MimeType,
}
impl UploadFileInfoFiled {
    fn get_all() -> Vec<UploadFileInfoFiled> {
        vec![
            UploadFileInfoFiled::Name,
            UploadFileInfoFiled::RelativePath,
            UploadFileInfoFiled::Size,
            UploadFileInfoFiled::MimeType,
        ]
    }
    fn to_str_filed(&self) -> &str {
        match self {
            UploadFileInfoFiled::Name => "name",
            UploadFileInfoFiled::RelativePath => "relative_path",
            UploadFileInfoFiled::Size => "size",
            UploadFileInfoFiled::MimeType => "mime_type",
        }
    }
}

pub type UploadMapFiled = HashMap<UploadFileInfoFiled, String>;

impl UploadFileInfo {
    fn get_value_by_map(&self, filed: UploadFileInfoFiled) -> Value {
        match filed {
            UploadFileInfoFiled::Name => json!(self.name),
            UploadFileInfoFiled::RelativePath => {
                #[cfg(target_os = "windows")]
                {
                    let relative_path = match self.relative_path.to_str() {
                        Some(s) => s.replace("\\", "/"),
                        None => {
                            log::error!("path contains invalid UTF-8 characters");
                            panic!("path contains invalid UTF-8 characters");
                        }
                    };
                    json!(relative_path)
                }
                #[cfg(not(target_os = "windows"))]
                {
                    json!(self.relative_path)
                }
            }
            UploadFileInfoFiled::Size => json!(self.size),
            UploadFileInfoFiled::MimeType => json!(self.mime_type),
        }
    }
    fn to_body(self, config: &FileUploadProcessParams) -> TardisResult<Value> {
        let mut value = json!({});

        for filed in UploadFileInfoFiled::get_all() {
            if let Some(map_filed) = &config.upload_metadata_rename_filed {
                if let Some(a) = map_filed.get(&filed) {
                    value
                        .as_object_mut()
                        .expect("can't be here")
                        .insert(a.to_string(), self.get_value_by_map(filed));
                    continue;
                }
            }

            value.as_object_mut().expect("can't be here").insert(
                filed.to_str_filed().to_string(),
                self.get_value_by_map(filed),
            );
        }
        if let Some(fixed_fileds) = &config.upload_fixed_metadata {
            for fixed_filed in fixed_fileds {
                value
                    .as_object_mut()
                    .expect("can't be here")
                    .insert(fixed_filed.0.to_string(), fixed_filed.1.clone());
            }
        }

        Ok(value)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadStatsResp {
    pub total_file_numbers: usize,
    pub total_file_size: u64,
}

pub static BACKGROUND_TASK: Lazy<Mutex<Option<TokioJoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(None));

pub async fn upload_files(
    files_uris: Vec<String>,
    window: Window,
) -> TardisResult<UploadStatsResp> {
    let mut total_file_numbers = 0;
    let mut total_file_size: u64 = 0;

    let param = crate::get_params();
    if let Some(upload) = param.upload {
        let mut files = Vec::new();
        for file_uri in files_uris {
            let origin_path = PathBuf::from(&file_uri);
            let base_path = origin_path.parent().unwrap_or(Path::new(""));
            let paths = async_get_files(&file_uri).await?;
            for path in paths {
                let relative_path = path
                    .strip_prefix(&base_path)
                    .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?;
                if path.is_file() {
                    let mime_type = mime_infer::from_path(path.clone()).first_or_text_plain();
                    let file = File::open(path.clone())
                        .await
                        .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?;
                    let size;
                    #[cfg(any(target_os = "macos", target_os = "linux"))]
                    {
                        size = file.metadata().await?.size();
                    }
                    #[cfg(target_os = "windows")]
                    {
                        size = file.metadata().await?.file_size();
                    }
                    let info = UploadFileInfo {
                        name: path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or_default()
                            .to_string(),
                        relative_path: relative_path.to_path_buf(),
                        size,
                        mime_type: mime_type.to_string(),
                        id: random::<u64>().to_string(),
                    };

                    files.push((Some(file), info));
                } else {
                    files.push((
                        None,
                        UploadFileInfo {
                            name: "".to_string(),
                            relative_path: relative_path.to_path_buf(),
                            size: 0,
                            mime_type: "dir".to_string(),
                            id: random::<u64>().to_string(),
                        },
                    ));
                }
            }
        }
        total_file_numbers = files.len();
        total_file_size = stream::iter(&files)
            .then(|(file, _)| get_metadata_size(file))
            .collect::<Vec<u64>>()
            .await
            .into_iter()
            .sum();
        let back_task;
        if param.title.eq("请按使用文档调用（以下为示例）") {
            //mock
            back_task = spawn(async move {
                mock_backend_task(files, total_file_numbers, total_file_size, window, upload).await;
            });
        } else {
            back_task = spawn(async move {
                backend_task(files, total_file_numbers, total_file_size, window, upload).await
            });
        }
        let mut guard = BACKGROUND_TASK
            .try_lock()
            .ok_or(TardisError::io_error(&format!("try lock error"), "error"))?;
        *guard = Some(back_task);
    }

    Ok(UploadStatsResp {
        total_file_numbers: total_file_numbers,
        total_file_size: total_file_size,
    })
}

async fn mock_backend_task(
    files: Vec<(Option<File>, UploadFileInfo)>,
    total_file_numbers: usize,
    total_file_size: u64,
    window: Window,
    _config: FileUploadProcessParams,
) {
    let mut uploaded_file_numbers = 0;
    let mut uploaded_file_size = 0;

    let mut last_file: Option<UploadFileInfo> = None;
    for (_file, info) in files {
        tardis::tokio::time::sleep(Duration::from_secs(1)).await;
        uploaded_file_numbers += 1;
        uploaded_file_size += info.size;
        if let Some(last_file) = &last_file {
            if random() {
                window
                    .emit(
                        "upload-progress",
                        UploadProgressResp {
                            uploaded_file_numbers,
                            uploaded_file_size,
                            current_files: vec![info.clone()],
                            fail_files: vec![last_file.clone()],
                            success_files: vec![],
                        },
                    )
                    .unwrap();
            } else {
                window
                    .emit(
                        "upload-progress",
                        UploadProgressResp {
                            uploaded_file_numbers,
                            uploaded_file_size,
                            current_files: vec![info.clone()],
                            fail_files: vec![],
                            success_files: vec![last_file.clone()],
                        },
                    )
                    .unwrap();
            }
        } else {
            window
                .emit(
                    "upload-progress",
                    UploadProgressResp {
                        uploaded_file_numbers,
                        uploaded_file_size,
                        current_files: vec![info.clone()],
                        fail_files: vec![],
                        success_files: vec![],
                    },
                )
                .unwrap();
        }
        last_file = Some(info);
    }

    window
        .emit(
            "upload-progress",
            UploadProgressResp {
                uploaded_file_numbers: total_file_numbers,
                uploaded_file_size: total_file_size,
                current_files: vec![],
                fail_files: vec![],
                success_files: if last_file.is_some() {
                    vec![last_file.unwrap()]
                } else {
                    vec![]
                },
            },
        )
        .unwrap();
}

async fn backend_task(
    files: Vec<(Option<File>, UploadFileInfo)>,
    total_file_numbers: usize,
    total_file_size: u64,
    window: Window,
    config: FileUploadProcessParams,
) {
    let mut uploaded_file_numbers = 0;
    let mut uploaded_file_size = 0;

    // first boolean means end(true)/start
    // seconde boolean is success(true)/fail
    let (tx, mut rx) = mpsc::channel(50);
    let max_concurrent_tasks = TardisFuns::cs_config::<ProcessorConfig>(DOMAIN_CODE).concurrent;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

    for (file, info) in files {
        tardis::tokio::task::yield_now().await;
        let n_tx = tx.clone();
        let config = config.clone();
        let semaphore = semaphore.clone();

        spawn(async move {
            let permit = semaphore.acquire_owned().await.unwrap();
            let _ = n_tx.send(((false, false), info.clone())).await;
            let body = info.clone().to_body(&config).unwrap();
            info!("file.body:{}", body);
            if let Some(mut file) = file {
                if let Ok(upload_metadata_result) = TardisFuns::web_client()
                    .post_obj_to_str(
                        config.upload_metadata_url,
                        &body,
                        config.upload_fixed_headers.unwrap_or_default(),
                    )
                    .await
                {
                    info!("upload_metadata_result:{:?}", upload_metadata_result);
                    if upload_metadata_result.code == 200 {
                        if let Some(upload_url) = upload_metadata_result.body {
                            info!("upload_url:{:?}", upload_url);
                            if reqwest::Url::parse(&upload_url).is_err() {
                                let _ = n_tx.send(((true, false), info.clone())).await;
                                return;
                            }

                            let mut content = vec![];
                            let _ = file.read_to_end(&mut content).await;
                            let client = reqwest::Client::new();
                            if let Ok(_) = client.put(upload_url).body(content).send().await {
                                let _ = n_tx.send(((true, true), info.clone())).await;
                                return;
                            }
                        }
                    }
                };
                let _ = n_tx.send(((true, false), info.clone())).await;
                drop(permit);
            } else {
                //empty dir
                let _ = TardisFuns::web_client()
                    .post_obj_to_str(
                        config.upload_metadata_url,
                        &body,
                        config.upload_fixed_headers.unwrap_or_default(),
                    )
                    .await;
                let _ = n_tx.send(((true, true), info.clone())).await;
            }
        });
    }

    let mut current_files_map = HashMap::new();
    while let Some(((is_done, is_success), i)) = rx.recv().await {
        let mut success_files = Vec::new();
        let mut fail_files = Vec::new();
        if uploaded_file_numbers == total_file_numbers {
            break;
        }
        if is_done {
            current_files_map.remove(&i.id);
            if is_success {
                success_files.push(i)
            } else {
                fail_files.push(i)
            }
        } else {
            uploaded_file_numbers += 1;
            uploaded_file_size += i.size;
            current_files_map.insert(i.id.clone(), i);
        }

        window
            .emit(
                "upload-progress",
                UploadProgressResp {
                    uploaded_file_numbers,
                    uploaded_file_size,
                    current_files: current_files_map
                        .iter()
                        .map(|(_, info)| info.clone())
                        .collect(),
                    fail_files,
                    success_files,
                },
            )
            .unwrap();
    }

    window
        .emit(
            "upload-progress",
            UploadProgressResp {
                uploaded_file_numbers: total_file_numbers,
                uploaded_file_size: total_file_size,
                current_files: vec![],
                fail_files: vec![],
                success_files: vec![],
            },
        )
        .unwrap();
}

async fn get_metadata_size(file: &Option<File>) -> u64 {
    if let Some(file) = file {
        file.metadata().await.map(|md| md.len()).unwrap_or_default()
    } else {
        0
    }
}

async fn async_get_files(file_uri: &str) -> TardisResult<Vec<PathBuf>> {
    let mut result = vec![];
    let path = PathBuf::from(file_uri);
    if path.is_file() {
        result.push(path);
    } else {
        let mut dir = read_dir(file_uri).await.expect("can't open dir");
        result.push(path);
        let mut push_dir = true;
        while let Some(d) = dir
            .next_entry()
            .await
            .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?
        {
            match d.path().to_str() {
                Some(path) => {
                    if push_dir {
                        result.remove(result.len() - 1);
                        push_dir = false
                    }
                    result.append(&mut Box::pin(async_get_files(path)).await?);
                }
                None => continue,
            };
        }
    }

    Ok(result)
}
