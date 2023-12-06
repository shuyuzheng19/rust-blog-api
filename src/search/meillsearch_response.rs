use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "hits")]
    pub hits: Vec<Hits>,
    pub offset: i64,
    pub limit: i64,
    #[serde(rename = "estimatedTotalHits")]
    pub total_hits: i64,
    pub query: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Formatted {
    pub description: String,
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Hits {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub _formatted: Formatted,
}
