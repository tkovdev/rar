use redis_macros_derive::{FromRedisValue, ToRedisArgs};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct User {
    pub first: String,
    pub last: String
}