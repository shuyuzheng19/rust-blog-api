use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
    #[serde(rename = "highlightPreTag")]
    highlight_pre_tag: String,
    #[serde(rename = "highlightPostTag")]
    highlight_post_tag: String,
    #[serde(rename = "showMatchesPosition")]
    show_matches_position: bool,
    sort: Vec<String>,
    #[serde(rename = "attributesToHighlight")]
    attributes_to_highlight: Vec<String>,
}

impl SearchQuery {
    pub fn new() -> Self {
        SearchQuery {
            q: String::new(),
            offset: None,
            limit: None,
            highlight_pre_tag: String::new(),
            highlight_post_tag: String::new(),
            show_matches_position: false,
            sort: Vec::new(),
            attributes_to_highlight: Vec::new(),
        }
    }

    pub fn set_attributes_to_highlight(mut self, highlight: Vec<String>) -> Self {
        self.attributes_to_highlight = highlight;
        self
    }

    pub fn set_show_matches_position(mut self, show: bool) -> Self {
        self.show_matches_position = show;
        self
    }

    pub fn set_q(mut self, q: String) -> Self {
        self.q = q;
        self
    }

    pub fn set_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn set_highlight_pre_tag(mut self, highlight_pre_tag: String) -> Self {
        self.highlight_pre_tag = highlight_pre_tag;
        self
    }

    pub fn set_highlight_post_tag(mut self, highlight_post_tag: String) -> Self {
        self.highlight_post_tag = highlight_post_tag;
        self
    }

    pub fn set_sort(mut self, sort: Vec<String>) -> Self {
        self.sort = sort;
        self
    }

    pub fn build(self) -> SearchQuery {
        self
    }
}
