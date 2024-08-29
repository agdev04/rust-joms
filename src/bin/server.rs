use rocket_db_pools::Database;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", rocket::routes![ 
        joms::routes::personal_information::get_personal_information,
        joms::routes::personal_information::get_single_personal_information,
        joms::routes::personal_information::create_personal_information,
        joms::routes::personal_information::update_personal_information,
        joms::routes::personal_information::delete_personal_information,
    ])
    .attach(joms::routes::Cors)
    .attach(joms::routes::DbConn::init())
    .launch()
    .await;

}