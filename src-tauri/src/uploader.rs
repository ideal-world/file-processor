use std::time::Duration;
use serde::{Deserialize, Serialize};
use tardis::{
    basic::result::TardisResult,
    tokio::{spawn, time::sleep},
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadStatsResp {
    pub total_file_numbers: u32,
    pub total_file_size: u64,
}

pub async fn upload_files(files_uri: &str, window: Window) -> TardisResult<UploadStatsResp> {
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
