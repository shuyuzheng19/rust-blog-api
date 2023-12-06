use actix_web::web;

use crate::controller;

pub fn category_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("category")
        .service(controller::category_controller::get_category_list_for_db)
        .service(controller::category_controller::get_category_list_for_cache)
        .service(controller::category_controller::add_category);
    conf.service(scope);
}

pub fn tag_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("tags")
        .service(controller::tag_controller::get_tag_list_for_db)
        .service(controller::tag_controller::get_random_tag_list)
        .service(controller::tag_controller::add_tag)
        .service(controller::tag_controller::get_tag_blogs)
        .service(controller::tag_controller::get_topic_by_id);
    conf.service(scope);
}

pub fn admin_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("admin")
        .service(controller::admin_controller::get_current_user_delete_blog_list)
        .service(controller::admin_controller::get_super_all_delete_blogs)
        .service(controller::admin_controller::get_current_user_blog_list)
        .service(controller::admin_controller::get_super_all_blogs)
        .service(controller::admin_controller::delete_blog_by_id)
        .service(controller::admin_controller::un_delete_blog_by_id)
        .service(controller::admin_controller::batch_delete_blog_by_ids)
        .service(controller::admin_controller::batch_un_delete_blog_by_ids)
        .service(controller::admin_controller::get_all_tag_list)
        .service(controller::admin_controller::get_category_list)
        .service(controller::admin_controller::batch_un_delete_category_by_ids)
        .service(controller::admin_controller::un_delete_category_by_id)
        .service(controller::admin_controller::delete_category_by_id)
        .service(controller::admin_controller::batch_delete_category_by_ids)
        .service(controller::admin_controller::batch_delete_tag_by_ids)
        .service(controller::admin_controller::batch_un_delete_tag_by_ids)
        .service(controller::admin_controller::delete_tag_by_id)
        .service(controller::admin_controller::un_delete_tag_by_id)
        .service(controller::admin_controller::update_category)
        .service(controller::admin_controller::update_tag)
        .service(controller::admin_controller::get_topic_list)
        .service(controller::admin_controller::get_topic_current_list)
        .service(controller::admin_controller::update_topic)
        .service(controller::admin_controller::get_file_list)
        .service(controller::admin_controller::update_file_public)
        .service(controller::admin_controller::set_recommend_blog)
        .service(controller::admin_controller::set_web_site_info)
        .service(controller::admin_controller::update_role)
        .service(controller::admin_controller::set_gpt_token)
        .service(controller::admin_controller::delete_file_by_id)
        .service(controller::admin_controller::delete_file_by_ids)
        .service(controller::admin_controller::delete_topic_by_id)
        .service(controller::admin_controller::delete_topic_by_ids)
        .service(controller::admin_controller::un_delete_topic_by_id)
        .service(controller::admin_controller::un_delete_topic_by_ids)
        .service(controller::admin_controller::init_search_blog)
        .service(controller::admin_controller::init_latest_blog)
        .service(controller::admin_controller::init_blog_count)
        .service(controller::admin_controller::get_log_info);
    conf.service(scope);
}

pub fn topic_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("topics")
        .service(controller::topic_controller::get_topic_list)
        .service(controller::topic_controller::get_topic_blogs)
        .service(controller::topic_controller::get_topic_by_id)
        .service(controller::topic_controller::get_user_topics)
        .service(controller::topic_controller::get_current_user_topic)
        .service(controller::topic_controller::get_all_topic_list)
        .service(controller::topic_controller::add_topic)
        .service(controller::topic_controller::get_all_topic_blogs);
    conf.service(scope);
}

pub fn file_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("file")
        .service(controller::file_controller::upload_file)
        .service(controller::file_controller::upload_avatar)
        .service(controller::file_controller::upload_image)
        .service(controller::file_controller::find_file_by_page_public)
        .service(controller::file_controller::find_file_by_page_current)
        .service(controller::file_controller::check_md5_and_insert_file);
    conf.service(scope);
}

pub fn user_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("user")
        .service(controller::user_controller::get_user_by_id)
        .service(controller::user_controller::login)
        .service(controller::user_controller::send_email_for_code)
        .service(controller::user_controller::registered_user)
        .service(controller::user_controller::contact_me)
        .service(controller::user_controller::get_web_site_info)
        .service(controller::user_controller::logout)
        .service(controller::user_controller::is_cn)
        .service(controller::user_controller::chat_gpt);
    conf.service(scope);
}

pub fn blog_router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("blog")
        .service(controller::blog_controller::get_blog_info_by_id)
        .service(controller::blog_controller::get_blog_by_category_list)
        .service(controller::blog_controller::get_hot_blogs_list)
        .service(controller::blog_controller::get_range_blog_list)
        .service(controller::blog_controller::get_latest_blogs_list)
        .service(controller::blog_controller::get_blog_by_user_list)
        .service(controller::blog_controller::get_user_top_blog)
        .service(controller::blog_controller::create_search_index)
        .service(controller::blog_controller::search_blog_list)
        .service(controller::blog_controller::init_search_blog)
        .service(controller::blog_controller::get_similar_blog)
        .service(controller::blog_controller::get_recommend_blog)
        .service(controller::blog_controller::save_blog)
        .service(controller::blog_controller::get_edit_blog)
        .service(controller::blog_controller::update_blog)
        .service(controller::blog_controller::set_save_edit_blog_content)
        .service(controller::blog_controller::get_save_edit_blog_content);
    conf.service(scope);
}
