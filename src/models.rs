use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug,AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable,AsChangeset)]
// #[belongs_to(User)]
// #[belongs_to(Profile)]
// #[has_many]
pub struct Profile{
    pub id: i32,
     pub profilename: String,
          pub created_at: chrono::NaiveDateTime,
          pub userr_id:i32,


}
#[derive(Insertable, Debug)]
#[table_name = "profiles"]
pub struct NewProfile<'a> {
    //pub profileid:&'a i32,
    pub userr_id:&'a i32,
    pub profilename: &'a str,
    pub created_at: chrono::NaiveDateTime,
}