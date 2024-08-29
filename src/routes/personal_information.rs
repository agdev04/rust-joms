use crate::models::{NewPersonalInformation, PersonalInformation};
use crate::repositories::PersonalInformationRepo;
use crate::routes::{DbConn, server_error};
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/personal-information")]
pub async fn get_personal_information(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    PersonalInformationRepo::all(&mut db, 100).await
    .map(|personal_information| json!(personal_information))
    .map_err(|e| server_error(e.into()))
}

#[rocket::get("/personal-information/<id>")]
pub async fn get_single_personal_information(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    PersonalInformationRepo::find(&mut db, id).await
    .map(|personal_information| json!(personal_information))
    .map_err(|e| server_error(e.into()))
}

#[rocket::post("/personal-information", data = "<new_personal_information>", format = "json")]
pub async fn create_personal_information(mut db: Connection<DbConn>, new_personal_information: Json<NewPersonalInformation>) -> Result<Value, Custom<Value>> {
    PersonalInformationRepo::create(&mut db, new_personal_information.into_inner()).await
    .map(|personal_information| json!(personal_information))
    .map_err(|e| server_error(e.into()))
}

#[rocket::put("/personal-information/<id>", data = "<personal_information>", format = "json")]
pub async fn update_personal_information(mut db: Connection<DbConn>, id: i32, personal_information: Json<PersonalInformation>) -> Result<Value, Custom<Value>> {
    PersonalInformationRepo::update(&mut db, id, personal_information.into_inner()).await
    .map(|personal_information| json!(personal_information))
    .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/personal-information/<id>")]
pub async fn delete_personal_information(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    PersonalInformationRepo::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|e| server_error(e.into()))
}