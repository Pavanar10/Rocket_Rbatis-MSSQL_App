use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct UserDetails {
    pub user_id: Option<i32>,
    pub user_name: String,
    pub email: String,
}



