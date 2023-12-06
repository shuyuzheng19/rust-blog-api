use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo<T> {
    pub page: i64,
    pub size: i64,
    pub total: i64,
    pub data: Vec<T>,
}
