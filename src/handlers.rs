// use actix_web::Responder;

// pub async fn get_users() -> impl Responder {
//     format!("hello from get users")
// }

// pub async fn get_user_by_id() -> impl Responder {
//     format!("hello from get users by id")
// }

// pub async fn add_user() -> impl Responder {
//     format!("hello from add user")
// }

// pub async fn delete_user() -> impl Responder {
//     format!("hello from delete user")
// }

use super::models::{NewUser, User, Profile, NewProfile};
use super::errors::{CustomError,map_io_error};
use super::schema::users::dsl::*;
use super::schema::profiles::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into,update};
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub id : i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputProfile {
    pub profileName: String,
    //pub userr_id:i32,
}


pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}
// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        id: &item.id,
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

pub async fn get_profiles(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_profiles(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_profiles(pool: web::Data<Pool>) -> Result<Vec<Profile>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = profiles.load::<Profile>(&conn)?;
    Ok(items)
}

pub async fn add_profile(
    db: web::Data<Pool>,
    item: web::Json<InputProfile>,profile_userr_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
     let mut s = String::new();
    Ok(web::block(move || add_single_profile(db, item,profile_userr_id.into_inner()))
        .await
        .map(|profile| HttpResponse::Created().json(profile))
        .map_err(|_| HttpResponse::InternalServerError().json({"User Not found"}))?)
}
fn add_single_profile(
    db: web::Data<Pool>,
    item: web::Json<InputProfile>,/*profile_userr_id:i32*/ user_id:i32,
) -> Result<Profile, diesel::result::Error> {
    let conn = db.get().unwrap();
    //let myusers = users.find(user_id).first::<User>(&conn).expect("Error loading user");
    let myprofile = users.find(user_id).first::<User>(&conn).expect("Error loading user");
    //let myUser= profile.find(user_id);
    //println!("{:?}", myusers.id);
    println!("{:?}", myprofile.id);
    let new_profile = NewProfile {
        userr_id:&myprofile.id,
        profilename: &item.profileName,
        created_at: chrono::Local::now().naive_local(),
    };
    // let res = insert_into(profiles).find(myprofile.userr_id).values(&new_profile).get_result(&conn)?;
let res = insert_into(profiles).values(&new_profile).get_result(&conn)?;
    Ok(res)
}




// pub async fn find_profile(
//     db: web::Data<Pool>,
//     item: web::Json<InputProfile>,profile_userr_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//      let mut s = String::new();
//     Ok(web::block(move || get_single_profile(db, item,profile_userr_id.into_inner()))
//         .await
//         .map(|profile| HttpResponse::Created().json(profile))
//         .map_err(|_| HttpResponse::InternalServerError().json({"User Not found"}))?)
// }
// fn get_single_profile(
//     db: web::Data<Pool>,
//     item: web::Json<InputProfile>,/*profile_userr_id:i32*/ user_id:i32,
// ) -> Result<Profile, diesel::result::Error> {
//     let conn = db.get().unwrap();
//     //let myusers = users.find(user_id).first::<User>(&conn).expect("Error loading user");
//     let myprofile = users.find(user_id).first::<User>(&conn).expect("Error loading user");
//     //let myUser= profile.find(user_id);
//     //println!("{:?}", myusers.id);
//     //println!("{:?}", myprofile.id);
//     // let new_profile = NewProfile {
//     //     userr_id:&myprofile.id,
//     //     profilename: &item.profileName,
//     //     created_at: chrono::Local::now().naive_local(),
//     // };
//     // let res = insert_into(profiles).find(myprofile.userr_id).values(&new_profile).get_result(&conn)?;
// // let res = insert_into(profiles).values(&new_profile).get_result(&conn)?;
//     profiles.find(&myprofile.id).get_result::<Profile>(&conn)
//     //Ok(res)
// }


pub async fn get_profile_by_id(
    db: web::Data<Pool>,
    profile_userr_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_profile_by_id(db, profile_userr_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError().json({"Profile Not found. Create a Profile."}))?,
    )
}


fn db_get_profile_by_id(pool: web::Data<Pool>, profile_userrr_id: i32) -> Result<Vec <Profile>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    //println!("{:?}",user_id);
    //let myusers = users.find(user_id).first::<User>(&conn).expect("Error loading user");
    //profiles.find(m).get_result::<Profile>(&conn)
    //let myprofile = users.find(profile_userr_id).first::<User>(&conn).expect("Error loading user");
    //let Profile=&myprofile.id;
    //let myprofiles = profiles.find(myprofile.userr_id)::<Profile>(&conn).expect("Error loading user");
    //Ok(myprofiles)
   // println!("{:?}",myprofile.userr_id);
//    let new_profile = Profile {
//         userr_id:myprofile.id,
//     };
    let myprofile = users.find(profile_userrr_id).first::<User>(&conn).expect("Error loading user");
    println!("{:?}",myprofile.id);
    //let myprofile = profiles.find(profile_userr_id).first::<Profile>(&conn).expect("Error loading user");
    let items = profiles.filter(userr_id.eq(myprofile.id)).load::<Profile>(&conn)?;
    Ok(items)
    //profiles.find(&myprofile.id).get_result::<Profile>(&conn)
    //println!("{:?}",mypf)
    //Ok(myprofile)
}



// pub async fn update_user_by_id(
//     db: web::Data<Pool>,
//     id: web::Path<i32>,
//     user: web::Json<InputUser>,
// ) -> Result<HttpResponse, Error> {
//     Ok(
//         web::block(move || db_update_user_by_id(db,id.into_inner(), employee.into_inner()))
//             .await
//             .map(|user| HttpResponse::Ok().json(user))
//             .map_err(|_| HttpResponse::InternalServerError().json({"Profile Not found. Create a Profile."}))?,
//     )
// }


// fn db_update_user_by_id(pool: web::Data<Pool>,id:i32, user:InputUser) -> Result<Vec <User>, diesel::result::Error> {
//     let conn = pool.get().unwrap();
//     //let myuser = users.find(user_id).first::<User>(&conn).expect("Error loading user");
//     //println!("{:?}",myprofile.id);
//     //let myprofile = profiles.find(profile_userr_id).first::<Profile>(&conn).expect("Error loading user");
//     let items = update(users).filter(id.eq(id)).set(user).get_result(&conn)?;
//     Ok(items)
//     //profiles.find(&myprofile.id).get_result::<Profile>(&conn)
//     //println!("{:?}",mypf)
//     //Ok(myprofile)
// }