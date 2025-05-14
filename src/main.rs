





#[macro_use]
extern crate rocket;
pub mod connection_model;

use crate::connection_model::connection::Connection;


#[launch]
fn rocket()->_{
    let conn: Connection = Connection::create_and_init();
    rocket::build().mount("/", routes![])
}