use log::error;
use sqlx::{Error, Pool, Postgres, Row};

use crate::common::constants::DEFAULT_ROLE_ID;
use crate::error::custom_error::{E, Status};
use crate::models::user::UserVo;
use crate::request::user_request::UserRegisteredRequest;

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(db_pool: Pool<Postgres>) -> UserRepository {
        return UserRepository { pool: db_pool };
    }

    /// 插入新用户数据。
    pub async fn insert_user(&self, user: &UserRegisteredRequest) -> Option<E> {
        let sql = "INSERT INTO
        users(username, nick_name, password, email, icon, deleted_at, create_at, update_at, role_id)
        VALUES($1, $2, $3, $4, $5, null, now(), now(), $6)";

        let result = sqlx::query(sql)
            .bind(&user.username)
            .bind(&user.nick_name)
            .bind(&user.password)
            .bind(&user.email)
            .bind(&user.icon)
            .bind(&DEFAULT_ROLE_ID)
            .execute(&self.pool)
            .await;

        return match result {
            Ok(r) => {
                if r.rows_affected() > 0 {
                    None
                } else {
                    Some(E::error(
                        Status::DATABASE_ERROR,
                        String::from("添加用户失败"),
                    ))
                }
            }
            Err(e) => {
                error!("{}", e);
                Some(E::error(
                    Status::DATABASE_ERROR,
                    String::from("添加用户==> DB执行出错"),
                ))
            }
        };
    }

    /// 通过用户ID获取用户信息。
    pub async fn get_user_by_id(&self, id: i64) -> Result<UserVo, Error> {
        let sql = "SELECT u.id, u.username, u.nick_name, u.password, u.icon , r.name as role_name
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id WHERE u.id = $1";

        let result = sqlx::query_as::<_, UserVo>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        return result;
    }

    /// 通过用户名获取用户信息。
    pub async fn get_user_by_username(&self, username: &String) -> Result<UserVo, Error> {
        let sql = "SELECT u.id, u.username, u.nick_name, u.password, u.icon , r.name as role_name
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id WHERE u.username = $1 LIMIT 1";

        let result = sqlx::query_as::<_, UserVo>(sql)
            .bind(username)
            .fetch_one(&self.pool)
            .await;

        return result;
    }

    /// 检查用户名或邮箱是否已存在。
    pub async fn user_is_exists(&self, username: &String, email: &String) -> Option<E> {
        let sql = "SELECT COUNT(id) FROM users WHERE username = $1 OR email = $2";

        let result = sqlx::query(sql)
            .bind(username)
            .bind(email)
            .fetch_one(&self.pool)
            .await;

        return match result {
            Ok(r) => {
                let count: i64 = r.get("count");
                if count > 0 {
                    Some(E::error(
                        Status::DATABASE_ERROR,
                        String::from("该账号或邮箱已存在."),
                    ))
                } else {
                    None
                }
            }
            Err(e) => {
                error!("{}", e);
                Some(E::error(
                    Status::DATABASE_ERROR,
                    String::from("用户是否存在==> DB执行出错"),
                ))
            }
        };
    }
}
