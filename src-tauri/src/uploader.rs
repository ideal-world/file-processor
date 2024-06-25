use std::time::Duration;

use serde::{Deserialize, Serialize};
use tardis::{
    basic::result::TardisResult,
    tokio::{fs::{read_dir, File}, spawn, time::sleep},
};
use tauri::{Manager, Window};

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadProgressResp {
    pub uploaded_file_numbers: u32,
    pub uploaded_file_size: u64,
    pub current_files: Vec<UploadFileInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadFileInfo {
    pub name: String,
    pub full_name: String,
    pub size: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq,Hash)]
pub enum  UploadFileInfoFiled{
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

pub async fn upload_files(files_uris: Vec<String>, window: Window) -> TardisResult<UploadStatsResp> {
  let param=crate::get_params();
  if let Some(upload) = param.upload {
    for file_uri in &files_uris {
      let file=File::open(file_uri).await.expect("can't open file!");
      if file.metadata().await.unwrap().is_file() {
      log::info!("file===={:?}",file.metadata().await);
      }else {
      let mut dir=read_dir(file_uri).await.expect("can't open dir");
      while let Some(d) = dir.next_entry().await.unwrap(){
        log::info!("d===={:?}",d.metadata().await);
      }
      }
    }
  
  }
    // Mock
    sleep(Duration::from_secs(1)).await;
    let total_file_numbers = 10000;
    let total_file_size = 102410000;
    spawn(async move {
        let mut uploaded_file_numbers = 0;
        let mut uploaded_file_size = 0;
        loop {
            if uploaded_file_numbers >= total_file_numbers {
                window
                    .emit(
                        "upload-progress",
                        UploadProgressResp {
                            uploaded_file_numbers: total_file_numbers,
                            uploaded_file_size: total_file_size,
                            current_files: vec![],
                        },
                    )
                    .unwrap();
                break;
            }
            sleep(Duration::from_millis(1000)).await;
            let current_files = vec![
                UploadFileInfo {
                    name: format!("file{}", uploaded_file_numbers + 1),
                    full_name: format!("a/b/file{}", uploaded_file_numbers + 1),
                    size: 1024,
                },
                UploadFileInfo {
                    name: format!("file{}", uploaded_file_numbers + 2),
                    full_name: format!("a/b/file{}", uploaded_file_numbers + 2),
                    size: 1024,
                },
            ];
            window
                .emit(
                    "upload-progress",
                    UploadProgressResp {
                        uploaded_file_numbers,
                        uploaded_file_size,
                        current_files,
                    },
                )
                .unwrap();
            uploaded_file_numbers += 2;
            uploaded_file_size += 2048;
        }
    });

    Ok(UploadStatsResp {
        total_file_numbers: total_file_numbers,
        total_file_size: total_file_size,
    })
}
