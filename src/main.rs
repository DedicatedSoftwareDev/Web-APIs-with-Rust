use diesel::{ PgConnection};
use rocket_sync_db_pools::database;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
mod models;
mod repositories;
mod rocket_routes;
mod schema;
#[database("postgres")]
pub struct DbConn(PgConnection);
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustacean,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
            ],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
