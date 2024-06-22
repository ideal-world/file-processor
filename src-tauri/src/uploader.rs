use serde::{Deserialize, Serialize};
use tardis::{basic::result::TardisResult, tokio::spawn};
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
    spawn(async move {
      // loop
        window
            .emit(
                "upload-progress",
                UploadProgressResp {
                    uploaded_file_numbers: 0,
                    uploaded_file_size: 0,
                    current_files: vec![],
                },
            )
            .unwrap();
    });
    Ok(UploadStatsResp {
        total_file_numbers: 0,
        total_file_size: 0,
    })
}
