use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};

use crate::common::constants::SEARCH_BLOG_PAGE_SIZE;
use crate::error::custom_error::{E, Status};
use crate::request::blog_request::SearchQueryRequest;
use crate::search::meillsearch_response::SearchResponse;

pub struct MeiliSearchClient {
    uri: String,
    headers: HeaderMap,
}

impl MeiliSearchClient {
    pub fn new(host: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/json;charset=utf-8"),
        );
        MeiliSearchClient {
            uri: host.to_string(),
            headers,
        }
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<Response, E> {
        let url = format!("{}/{}", self.uri, endpoint);

        let client = Client::new();

        let request = client.request(method, &url).headers(self.headers.clone());

        let request = if let Some(body) = body {
            request.json(&body)
        } else {
            request
        };

        let response = request.send().await.unwrap();

        return Ok(response);
    }

    pub async fn create_index(&self, index: &str) -> Option<E> {
        let endpoint = "indexes";
        let payload = json!({
            "uid":index,
            "primaryKey":"id"
        });
        match self
            .send_request(reqwest::Method::POST, endpoint, Some(payload))
            .await
        {
            Ok(_) => None,
            Err(_) => Some(E::error(
                Status::HTTP_REQUEST_ERROR,
                String::from("创建索引失败"),
            )),
        }
    }

    pub async fn drop_index(&self, index: &str) -> Option<E> {
        let endpoint = format!("indexes/{}", index);
        match self
            .send_request(reqwest::Method::DELETE, &endpoint, None)
            .await
        {
            Ok(_) => None,
            Err(_) => Some(E::error(
                Status::HTTP_REQUEST_ERROR,
                String::from("删除索引失败"),
            )),
        }
    }

    pub async fn delete_all_documents(&self, index: &str) -> Option<E> {
        let endpoint = format!("indexes/{}/documents", index);
        match self
            .send_request(reqwest::Method::DELETE, &endpoint, None)
            .await
        {
            Ok(_) => None,
            Err(_) => Some(E::error(
                Status::HTTP_REQUEST_ERROR,
                String::from("删除所有文档失败"),
            )),
        }
    }

    pub async fn save_documents(&self, index: &str, document_json: Value) -> Option<E> {
        let endpoint = format!("indexes/{}/documents", index);
        match self
            .send_request(reqwest::Method::POST, &endpoint, Some(document_json))
            .await
        {
            Ok(_) => None,
            Err(_) => Some(E::error(
                Status::HTTP_REQUEST_ERROR,
                String::from("添加文档失败"),
            )),
        }
    }

    pub async fn search_documents(
        &self,
        index: &str,
        req: &SearchQueryRequest,
    ) -> Option<SearchResponse> {
        let endpoint = format!("indexes/{}/search", index);
        let size = SEARCH_BLOG_PAGE_SIZE;

        let search_request = json!({
            "q": req.keyword,
            "offset": (req.page - 1) * size,
            "attributesToHighlight": ["*"],
            "limit": size,
            "showMatchesPosition": false,
            "highlightPreTag": "<b>",
            "highlightPostTag": "</b>"
        });

        let response = self
            .send_request(reqwest::Method::POST, &endpoint, Some(search_request))
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let search_response: SearchResponse =
                        serde_json::from_str(&response.text().await.unwrap()).unwrap();
                    return Some(search_response);
                } else {
                    return None;
                }
            }
            Err(_) => None,
        }
    }
}
