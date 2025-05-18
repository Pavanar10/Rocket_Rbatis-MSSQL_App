use rocket::State;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};
use serde_json::json;
use crate::connection_model::connection::Connection;
use crate::model::user::userdetails;
use crate::connection_model::user::UserConnection;


#[get("/user")]
pub async fn get_users(conn: &State<Connection>) -> String {
    UserConnection::get_users(conn).await
}

/*#[patch("/AEO/update-dossier-vervoerder/<id>")]
pub async fn update_dossier_vervoerder(conn: &State<Connection>,id:u64) -> String {
    DossierConnection::update_dossier(conn, id).await
}*/

/*#[patch("/update_user",data ="<user>")]
pub async fn update_user(conn: &State<Connection>, user: Json<User>) -> String {
    UserConnection::update_user(conn, &user).await
}



#[post("/add_user", data = "<user>")]
pub async fn add_user<'r>(conn: &State<Connection>, user: Json<User>) -> status::Created<String> {
    let result: String = UserConnection::add_user(conn, user).await;
    let url = "http://vrynt812:4200/spekkies".to_string();
    status::Created::new(url).tagged_body(result)
}*/