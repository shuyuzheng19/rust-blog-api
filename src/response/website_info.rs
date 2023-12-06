use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub icon: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "modalImage")]
    pub modal_image: Option<String>,
    pub href: Option<String>,
    pub modal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogConfigInfo {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub icon: Vec<Icon>,
    #[serde(rename = "musicId")]
    pub music_id: Option<String>,
    pub descriptions: Option<Vec<String>>,
    pub content: Option<String>,
}

pub fn get_default_blog_config_info() -> BlogConfigInfo {
    BlogConfigInfo {
        name: Some("".to_string()),
        avatar: Some("".to_string()),
        icon: vec![
            Icon {
                icon: Some("".to_string()),
                title: Some("点击显示我的二维码".to_string()),
                modal_image: Some("".to_string()),
                href: Some("".to_string()),
                modal: false,
            },
            Icon {
                icon: Some("".to_string()),
                title: Some("".to_string()),
                modal_image: Some("".to_string()),
                href: Some("".to_string()),
                modal: false,
            },
            Icon {
                icon: Some("".to_string()),
                title: Some("我的Github地址".to_string()),
                modal_image: Some("".to_string()),
                href: Some("".to_string()),
                modal: false,
            },
            Icon {
                icon: Some("".to_string()),
                title: Some("".to_string()),
                modal_image: Some("".to_string()),
                href: Some("".to_string()),
                modal: false,
            },
        ],
        music_id: Some("".to_string()),
        descriptions: Some(vec![
            String::from("后端程序员一枚"),
            String::from("有好的需求可以联系作者"),
        ]),
        content: Some("".to_string()),
    }
}
