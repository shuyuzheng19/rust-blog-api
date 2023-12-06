// 用于存储页面相关的常量和默认页数的结构
pub struct PageConstants;

// 通用常量结构
pub struct Constant;

// 默认角色 ID
pub const DEFAULT_ROLE_ID: i64 = 1;

// 博客页面大小
pub const BLOG_PAGE_SIZE: i64 = 10;

// 热门博客页面大小
pub const HOT_BLOG_PAGE_SIZE: i64 = 10;

// 存档博客页面大小
pub const ARCHIVE_BLOG_PAGE_SIZE: i64 = 15;

// 最新博客页面大小
pub const LATEST_BLOG_PAGE_SIZE: i64 = 10;

// 用户置顶博客页面大小
pub const USER_TOP_BLOG_PAGE_SIZE: i64 = 10;

// 搜索博客页面大小
pub const SEARCH_BLOG_PAGE_SIZE: i64 = 10;

// 随机标签列表数量
pub const TAG_RANDOM_LIST_COUNT: usize = 20;

// 主题页面数量
pub const TOPIC_PAGE_COUNT: i64 = 20;

// 文件页面熟练
pub const FILE_PAGE_COUNT: i64 = 15;

// 后台管理博客页面数量
pub const ADMIN_BLOG_PAGE_COUNT: i64 = 10;

// 后台管理分类、标签页面数量
pub const CATEGORY_ADMIN_PAGE_COUNT: i64 = 15;

// 后台管理专题页面数量
pub const TOPIC_ADMIN_PAGE_COUNT: i64 = 15;

// 后台管理文件页面数量
pub const FILE_ADMIN_PAGE_COUNT: i64 = 15;

// 默认页数（用于分页）
pub fn default_page() -> i64 {
    1
}
