
#[macro_use]
extern crate rocket;
pub mod connection_model;
pub mod routes;
pub mod model;

use crate::connection_model::connection::Connection;
use crate::routes::user::{get_users,add_user,delete_user,update_user};


#[launch]
fn rocket()->_{
    let conn: Connection = Connection::create_and_init();
    rocket::build()
    .manage(conn)
    .mount("/", routes![get_users,add_user,delete_user,update_user])
}