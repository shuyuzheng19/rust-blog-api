use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadConfig {
    pub max_image_size: i64,
    pub max_file_size: i64,
    pub prefix: String,
    pub uri: String,
    pub path: String,
    pub avatar: String,
    pub image: String,
    pub files: String,
}
