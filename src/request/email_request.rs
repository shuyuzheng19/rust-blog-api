use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub message: String,
}
