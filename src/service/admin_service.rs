use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::cache::{
    clear_category_info_keys, clear_page_info_keys, clear_tag_info_key, clear_topic_info_key,
};
use crate::conf::config::CONFIG;
use crate::models::blogs::BlogAdminVo;
use crate::models::category::{CategoryVo, OtherAdminVo};
use crate::models::file::FileAdminVo;
use crate::models::topic::{AdminTopicVo, TopicRequest};
use crate::repository::admin_repository::AdminRepository;
use crate::request::admin_request::{
    AdminBlogFilter, OtherAdminFilter, UpdatePublicRequest, UpdateRole,
};
use crate::response::page_info::PageInfo;

pub struct AdminService(Arc<AdminRepository>);

impl AdminService {
    pub fn new(db_conn: Pool<Postgres>) -> AdminService {
        let tag_repository = AdminRepository::new(db_conn);
        AdminService(Arc::new(tag_repository))
    }

    pub async fn get_admin_blog_list(
        &self,
        req: AdminBlogFilter,
        deleted: bool,
        uid: i64,
    ) -> PageInfo<BlogAdminVo> {
        return self.0.get_admin_blog_filter(req, deleted, uid).await;
    }

    pub async fn delete_blog_ids(&self, ids: Vec<i64>, uid: i64, deleted: bool) -> i64 {
        let i = self
            .0
            .global_delete_by_ids("blogs", &ids, uid, deleted)
            .await;

        if i > 0 {
            if CONFIG.blog_page_cache {
                clear_page_info_keys()
            }
        }

        return i;
    }

    pub async fn delete_category_ids(&self, ids: &Vec<i64>, deleted: bool) -> i64 {
        let i = self
            .0
            .global_delete_by_ids("categories", ids, -1, deleted)
            .await;

        if i > 0 {
            clear_category_info_keys();
            &self.0.delete_blog_by_categories(ids, deleted, -1).await;
            if CONFIG.blog_page_cache {
                clear_page_info_keys()
            }
        }

        return i;
    }

    pub async fn delete_tag_ids(&self, ids: &Vec<i64>, deleted: bool) -> i64 {
        let i = self.0.global_delete_by_ids("tags", ids, -1, deleted).await;

        if i > 0 {
            clear_tag_info_key()
        }

        return i;
    }

    pub async fn delete_topics_ids(&self, ids: &Vec<i64>, deleted: bool, uid: i64) -> i64 {
        let i = self
            .0
            .global_delete_by_ids("topics", ids, uid, deleted)
            .await;

        if i > 0 {
            clear_topic_info_key();
            self.0.delete_blog_by_topics(ids, deleted, uid).await;
        }

        return i;
    }

    pub async fn get_admin_category_list(
        &self,
        filter: OtherAdminFilter,
    ) -> PageInfo<OtherAdminVo> {
        return self
            .0
            .get_admin_other_filter(String::from("categories"), filter)
            .await;
    }

    pub async fn get_admin_topic_list(
        &self,
        filter: OtherAdminFilter,
        uid: i64,
    ) -> PageInfo<AdminTopicVo> {
        return self.0.get_admin_topic_filter(filter, uid).await;
    }

    pub async fn get_admin_tag_list(&self, filter: OtherAdminFilter) -> PageInfo<OtherAdminVo> {
        return self
            .0
            .get_admin_other_filter(String::from("tags"), filter)
            .await;
    }

    pub async fn update_category(&self, c: CategoryVo) -> i64 {
        return self
            .0
            .update_category_or_tag_name(String::from("categories"), c)
            .await;
    }

    pub async fn update_tag(&self, c: CategoryVo) -> i64 {
        return self
            .0
            .update_category_or_tag_name(String::from("tags"), c)
            .await;
    }

    pub async fn update_topic(&self, topic: TopicRequest, uid: i64) -> i64 {
        return self.0.update_topic(topic, uid).await;
    }

    pub async fn get_admin_files(&self, req: OtherAdminFilter, uid: i64) -> PageInfo<FileAdminVo> {
        return self.0.get_file_list(req, uid).await;
    }

    pub async fn update_file_public(&self, req: UpdatePublicRequest, uid: i64) -> i64 {
        return self.0.update_file_public(req, uid).await;
    }

    pub async fn update_role(&self, req: &UpdateRole) -> i64 {
        return self.0.update_role(req).await;
    }

    pub async fn delete_file(&self, ids: Vec<i64>, uid: i64, force: bool) -> i64 {
        let result = self.0.delete_file_by_ids(ids, uid, force).await;
        return result;
        // let mut count = 0;
        // if uid==-1 && force{
        //     for path in result {
        //         if  fs::remove_file(path).is_ok(){
        //             count+=1;
        //         }
        //     }
        //     return count
        // }else{
        //     return result.len() as i64;
        // }
    }
}
