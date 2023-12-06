// 这些常量代表时间单位的秒数
pub const MIN: usize = 60 * 1;
pub const HOUR: usize = 60 * 60;
pub const DAY: usize = 60 * 60 * 24;

// 用户信息键
pub const USER_INFO_KEY: &str = "USER-INFO:";

// 热门博客键
pub const HOT_BLOG_KEY: &str = "HOT-BLOG";

// 最新博客键
pub const LATEST_BLOG_KEY: &str = "LATEST-BLOG";

// 博客映射键
pub const BLOG_MAP_KEY: &str = "BLOG-MAP";

// 邮件验证码键
pub const EMAIL_CODE_KEY: &str = "EMAIL-CODE:";

// 网站配置键
pub const BLOG_WEB_CONFIG: &str = "WEBSITE-CONFIG";

// 分类列表键
pub const CATEGORY_LIST_KEY: &str = "CATEGORY-LIST";

// 随机标签键
pub const RANDOM_TAG_KEY: &str = "RANDOM-TAG";

// 推荐博客键
pub const RECOMMEND_BLOG_KEY: &str = "RECOMMEND-BLOG-KEY";

// 第一页主题键
pub const FIRST_PAGE_TOPIC_KEY: &str = "FIRST-TOPIC-KEY";

// 博客列表分页信息键
pub const BLOG_LIST_PAGE_INFO_KEY: &str = "BLOG-PAGE-INFO:";

// 主题映射键
pub const TOPIC_MAP_KEY: &str = "TOPIC-MAP";

// 博客浏览次数映射键
pub const EYE_COUNT_MAP: &str = "BLOG_EYE_COUNT_MAP_KEY";

// 保存或编辑博客键
pub const SAVE_BLOG_MAP: &str = "SAVE-BLOG-MAP";

// 用户令牌键
pub const USER_TOKEN_KEY: &str = "USER-TOKEN:";

// 主题映射键（注意：与 TOPIC_MAP_KEY 重复）
pub const TAG_MAP_KEY: &str = "TOPIC-MAP";

// 博客列表分页信息键的过期时间（6小时）
pub const BLOG_LIST_PAGE_INFO_EXPIRE: usize = HOUR * 6;

// 用户信息键的过期时间（30分钟）
pub const USER_INFO_KEY_EXPIRE: usize = MIN * 30;

// 热门博客键的过期时间（30分钟）
pub const HOT_BLOG_KEY_EXPIRE: usize = MIN * 30;

// 最新博客键的过期时间（3小时）
pub const LATEST_BLOG_KEY_EXPIRE: usize = HOUR * 3;

// 邮件验证码键的过期时间（1分钟）
pub const EMAIL_CODE_KEY_EXPIRE: usize = MIN * 1;

// 第一页主题键的过期时间（8小时）
pub const FIRST_PAGE_TOPIC_EXPIRE: usize = HOUR * 8;
