use serde::{Deserialize, Serialize};
use tardis::basic::result::TardisResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadProgressResp {
    pub rate: i8,
}

pub async fn upload_files(files_uri: &str) -> TardisResult<bool> {
    Ok(true)
}

pub async fn get_progress() -> TardisResult<UploadProgressResp> {
    todo!()
}
