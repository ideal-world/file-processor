use std::{
    os::unix::fs::MetadataExt, path::{Path, PathBuf}, time::Duration
};

use mime_infer::MimeInfer;
use serde::{Deserialize, Serialize};
use tardis::{
    basic::{error::TardisError, result::TardisResult},
    futures::{future::BoxFuture, FutureExt as _},
    tokio::{
        fs::{read_dir, File},
        spawn,
        time::sleep,
    },
};
use tauri::{utils::mime_type, Manager, Window};

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadProgressResp {
    pub uploaded_file_numbers: u32,
    pub uploaded_file_size: u64,
    pub current_files: Vec<UploadFileInfo>,
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct UploadFileInfo {
    pub name: String,
    // Relative path
    pub relative_path: PathBuf,
    pub size: u64,
    pub mime_type:String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum UploadFileInfoFiled {
    Name,
    FullName,
    Size,
    MimeType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadStatsResp {
    pub total_file_numbers: u32,
    pub total_file_size: u64,
}

pub async fn upload_files(
    files_uris: Vec<String>,
    window: Window,
) -> TardisResult<UploadStatsResp> {
    let param = crate::get_params();
    if let Some(upload) = param.upload {
        for file_uri in files_uris {
          let origin_path=PathBuf::from(&file_uri);
            let base_path=origin_path.parent().unwrap_or(Path::new(""));
            let paths = get_files(&file_uri).await?;
            for path in paths {
                let mime_type = mime_infer::from_path(path.clone()).first_or_text_plain();
                let file = File::open(path.clone())
                    .await
                    .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?;
                let relative_path = path
                    .strip_prefix(&base_path)
                    .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?;
                  let info=UploadFileInfo{
                    name: path.file_name().and_then(|s|s.to_str()).unwrap_or_default().to_string(),
                    relative_path: relative_path.to_path_buf(),
                    size: file.metadata().await?.size(),
                    mime_type:mime_type.to_string(),
                };
                log::info!("file====info:{info:?}");
            }
        }
    }
    // Mock
    sleep(Duration::from_secs(1)).await;
    let total_file_numbers = 10000;
    let total_file_size = 102410000;
    // spawn(async move {
    //     let mut uploaded_file_numbers = 0;
    //     let mut uploaded_file_size = 0;
    //     loop {
    //         if uploaded_file_numbers >= total_file_numbers {
    //             window
    //                 .emit(
    //                     "upload-progress",
    //                     UploadProgressResp {
    //                         uploaded_file_numbers: total_file_numbers,
    //                         uploaded_file_size: total_file_size,
    //                         current_files: vec![],
    //                     },
    //                 )
    //                 .unwrap();
    //             break;
    //         }
    //         sleep(Duration::from_millis(1000)).await;
    //         let current_files = vec![
    //             UploadFileInfo {
    //                 name: format!("file{}", uploaded_file_numbers + 1),
    //                 relative_path: format!("a/b/file{}", uploaded_file_numbers + 1),
    //                 size: 1024,
    //             },
    //             UploadFileInfo {
    //                 name: format!("file{}", uploaded_file_numbers + 2),
    //                 relative_path: format!("a/b/file{}", uploaded_file_numbers + 2),
    //                 size: 1024,
    //             },
    //         ];
    //         window
    //             .emit(
    //                 "upload-progress",
    //                 UploadProgressResp {
    //                     uploaded_file_numbers,
    //                     uploaded_file_size,
    //                     current_files,
    //                 },
    //             )
    //             .unwrap();
    //         uploaded_file_numbers += 2;
    //         uploaded_file_size += 2048;
    //     }
    // });

    Ok(UploadStatsResp {
        total_file_numbers: total_file_numbers,
        total_file_size: total_file_size,
    })
}

fn get_files<'a>(files_uri: &'a str) -> BoxFuture<'a, TardisResult<Vec<PathBuf>>> {
    async_get_files(files_uri).boxed()
}

async fn async_get_files(file_uri: &str) -> TardisResult<Vec<PathBuf>> {
    let mut result = vec![];
    let path = PathBuf::from(file_uri);
    if path.is_file() {
        result.push(path);
    } else {
        let mut dir = read_dir(file_uri).await.expect("can't open dir");
        while let Some(d) = dir
            .next_entry()
            .await
            .map_err(|e| TardisError::io_error(&format!("io error:{e}"), "error"))?
        {
            match d.path().to_str() {
                Some(path) => result.append(&mut get_files(path).await?),
                None => continue,
            };
        }
    }

    Ok(result)
}
