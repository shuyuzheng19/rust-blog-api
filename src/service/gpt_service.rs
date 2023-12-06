use reqwest::{Client,Response};
use uuid::Uuid;

use crate::conf::config::CONFIG;
use crate::error::custom_error::{E, Status};

pub struct GptService {
    token: String,
    cookie: String
}

impl GptService {
    pub fn new() -> Self {
        return Self {
            token: CONFIG.gpt.token.to_owned(),
            cookie: CONFIG.gpt.cookie.to_owned()
        };
    }

    pub fn update_token(&mut self, t:String, keyword: String) {
        if t=="token"{
            self.token = keyword;
        }else{
            self.cookie=keyword
        }
    }

    pub async fn chat(&self, message: &str) -> Result<Response, E> {
        let id = Uuid::new_v4();
        let gpt_token = ""; // 替换为实际的GPT令牌
        let parent_message_id = Uuid::new_v4();
        let gpt_url = &CONFIG.gpt.api; // 替换为实际的GPT URL

        let request_json = format!(
            r#"{{"action":"next","messages":[{{"id":"{}","role":"user","content":{{"parts":["{}"],"content_type":"text"}}}}],"model":"{}","parent_message_id":"{}"}}"#,
            id, message, gpt_token, parent_message_id
        );

        let client = Client::new();

        let response = client
            .post(gpt_url)
            .header("X-Authorization","Bearer ".to_owned()+&self.token)
            .header("Cookie",&self.cookie)
            .header("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 Edg/119.0.0.0")
            .body(request_json)
            .send()
            .await
            .unwrap();

        let status = response.status();

        if status != reqwest::StatusCode::OK {
            return Err(E::error(
                Status::AUTHORIZED_ERROR,
                String::from("访问出错，可能token已过期或其他问题"),
            ));
        }
        Ok(response)
    }
}
