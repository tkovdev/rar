mod db;
mod user;

#[macro_use] extern crate rocket;

use redis::{Client, Commands, FromRedisValue, RedisResult};
use redis_macros_derive::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use user::{User};

#[get("/")]
fn index() -> String {
    format!("{}", "Hello, world! 3rd Test!")
}

#[get("/<userId>")]
fn get_user(userId: String) -> String {
    let mut client = redis::Client::open("redis://redis-test:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let key = format!("user:{}", userId);
    let stored_user: User = con.get(&key).unwrap();

    format!("{} {}", stored_user.first, stored_user.last)
}

#[post("/<userId>", format = "application/json", data = "<user>")]
fn create(userId: String, user: Json<User>) -> () {
    let client = redis::Client::open("redis://redis-test:6379/").unwrap();
    let mut con = client.get_connection().unwrap();

    let key = format!("user:{}", userId);

    // Just use it as you would a primitive
    let _: () = con.set(&key, &*user).expect("couldn't set user");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_user, create])
}
