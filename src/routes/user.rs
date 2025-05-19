use rocket::State;
use rocket::response::status;
use rocket::serde::{json::Json};
use crate::connection_model::connection::Connection;
use crate::model::user::UserDetails;
use crate::connection_model::user::UserConnection;


#[get("/user")]
pub async fn get_users(conn: &State<Connection>) -> String {
    UserConnection::get_users(conn).await
}

#[post("/add_user", data = "<user>")]
pub async fn add_user<'r>(conn: &State<Connection>, user: Json<UserDetails>) -> status::Created<String> {
    let result: String = UserConnection::add_user(conn, user.into_inner()).await;
    let url = "http://vrynt812:4200/spekkies".to_string();
    status::Created::new(url).tagged_body(result)
}

#[delete("/delete_user/<id>")]
pub async fn delete_user(conn: &State<Connection>,id:u32){
    UserConnection::delete_user(conn,id).await;
}

#[patch("/update_user",data="<user>")]
pub async fn update_user(conn: &State<Connection>,user: Json<UserDetails>)->String{
    let res = UserConnection::update_user(conn, user.into_inner()).await;
    res
}

/*#[patch("/update_user",data ="<user>")]
pub async fn update_user(conn: &State<Connection>, user: Json<User>) -> String {
    UserConnection::update_user(conn, &user).await
}



*/