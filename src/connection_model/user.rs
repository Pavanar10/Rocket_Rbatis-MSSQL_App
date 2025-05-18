use rbatis::{crud,RBatis,};
use rocket::State;
use serde_json::json;


use crate::model::user::userdetails;
use crate::Connection;

crud!(userdetails{});

pub struct UserConnection {}

impl UserConnection {
    pub async fn get_users(conn: &State<Connection>) -> String {
        println!("reached here ");

        let selected_request = userdetails::select_all(&conn.db).await;
        match selected_request {
            Ok(res)=>{
                
                println!("res{:?}",res);
                json!(res).to_string()},
            Err(e)=>json!({"error":e.to_string()}).to_string()
            
        }

       // json!(selected_request).to_string()
    }

   /* pub async fn get_user_by_id(conn: &State<Connection>, id: i64) -> User {
        User::select_by_column(&mut &conn.db, "id", id).await.unwrap().remove(0)
    }

//delete
pub async fn delete_user_by_id(conn: &State<Connection>, id: i64) {
    match User::delete_by_column(&mut &conn.db, "id", id).await {
        Ok(r) => r,
        Err(e) => {
            println!("Can't delete object  {}", e);
            return;
        }
    };
}

//patch
    pub async fn update_user(conn: &State<Connection>, mut user:User) -> String {
       // let id: u64 = dossier.id.expect("no id");
        //dossier.id = None;
        //dossier.fk_landscode_id=None;
        match User::update_by_column_value(&mut &conn.db, &user, "id", &rbs::to_value!(id)).await {
            Ok(r) => r,
            Err(e) => panic!("Can't update object {:?}", e)
        };
        json!(user).to_string()
    }

    pub async fn add_user(conn: &State<Connection>,mut user: User) -> String {
        //dossier.id=None;
        //dossier.fk_landscode_id=None;
        match User::insert(&mut &conn.db, &user).await {
            Ok(r) => r,
            Err(e) => panic!("Can't insert object {:?}", e)
        };
        let id: u64 = UserConnection::get_scope_identity(&conn.db).await;
        let mut created_dossier: Vec<User> = User::select_by_column(&mut &conn.db, "id", id).await.unwrap();
        json!(created_user.remove(0)).to_string()
    }

    pub async fn get_scope_identity(conn: &Rbatis) -> u64 {
        let id: u64 = conn
            .query_decode("select top 1 id from user order by id desc", vec![])
            .await
            .unwrap();
        id
    }*/
}
