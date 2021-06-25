#[macro_use] extern crate rocket;

use rocket_sync_db_pools::{database, diesel};

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api", routes![hello])
}
