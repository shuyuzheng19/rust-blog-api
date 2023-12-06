use serde::{Deserialize, Serialize};

use crate::search::meilisearch_client::MeiliSearchClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliSearchConfig {
    host: String,
    api_key: String,
}

impl MeiliSearchConfig {
    pub fn get_search_client(&self) -> MeiliSearchClient {
        MeiliSearchClient::new(&self.host, &self.api_key)
    }
}
