use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Local};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserVo2 {
    pub id: i64,
    pub username: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub role: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserVo {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub role: String,
    pub icon: String,
}

impl UserVo{
    pub fn to_vo(self)->UserVo2{
        return UserVo2{id:self.id,username:self.username,nick_name:self.nick_name,role:self.role,icon:self.icon}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleUserVo {
    pub id: i64,
    #[serde(rename = "nickName")]
    pub nick_name: String,
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub icon: Option<String>,
    pub deleted_at: Option<DateTime<Local>>,
    pub create_at: DateTime<Local>,
    pub update_at: DateTime<Local>,
}

impl<'c> FromRow<'c, PgRow> for UserVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        Ok(UserVo {
            id: row.get("id"),
            password: row.get("password"),
            username: row.get("username"),
            nick_name: row.get("nick_name"),
            role: row.get("role_name"),
            icon: row.get("icon"),
        })
    }
}
