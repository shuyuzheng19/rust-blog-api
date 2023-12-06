//
// impl RedisConfig{
//     pub async fn get_redis_con(&self)->Connection{
//         let config = &CONFIG.redis;
//         println!("{}",config.host);
//         let url = format!("redis://{}@{}:{}/{}",config.password,config.host,config.port,config.db);
//         let client = Client::open(url);
//         if client.is_ok() {
//             return client.unwrap().get_connection().unwrap()
//         }
//         panic!("redis 连接失败")
//     }
//
//     pub async fn get_async_redis_con(&self)->redis::aio::Connection{
//         let config = &CONFIG.redis;
//         let url = format!("redis://{}@{}:{}/{}",config.password,config.host,config.port,config.db);
//         let client = Client::open(url);
//         if client.is_ok() {
//             return client.unwrap().get_tokio_connection().await.unwrap();
//         }
//         panic!("redis 连接失败")
//     }
//
//     pub async fn get_redis_pool(&self)->Pool<RedisConnectionManager>{
//         let config = &CONFIG.redis;
//         let url = format!("redis://{}@{}:{}/{}",config.password,config.host,config.port,config.db);
//         let manager = RedisConnectionManager::new(url).unwrap();
//
//         let pool = Pool::builder()
//             .max_size(config.max_size)
//             .min_idle(Some(config.min_idle))
//             .build(manager)
//             .unwrap();
//
//         return pool
//     }
// }

use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};

use crate::conf::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    db: i32,
    port: i64,
    host: String,
    password: String,
    min_idle: u32,
    max_size: u32,
}

lazy_static! {
    static ref REDIS_POOL: Pool<RedisConnectionManager> = {
        let config: &RedisConfig = &CONFIG.redis;

        let url = format!(
            "redis://default:{}@{}:{}/{}",
            config.password, config.host, config.port, config.db
        );

        let manager = RedisConnectionManager::new(url).unwrap();

        let pool = Pool::builder()
            .max_size(config.max_size)
            .min_idle(Some(config.min_idle))
            .build(manager)
            .unwrap();

        return pool;
    };
}

pub fn get_pool_connection() -> PooledConnection<RedisConnectionManager> {
    return REDIS_POOL.get().unwrap();
}
