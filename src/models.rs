use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name=personal_information)]
pub struct PersonalInformation {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub name_ext: Option<String>,
    pub middle_name: Option<String>,
    pub birth_date: String,
    pub birth_place: String,
    pub sex: String,
    pub civil_status: String,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub bloodtype: Option<String>,
    pub gsis: Option<String>,
    pub pagibig: Option<String>,
    pub philhealth: Option<String>,
    pub sss: Option<String>,
    pub tin: Option<String>,
    pub agency_employee: Option<String>,
    pub citizenship: Option<String>,
    pub ra_lot_no: Option<String>,
    pub ra_street: Option<String>,
    pub ra_subdivision: Option<String>,
    pub ra_barangay: Option<String>,
    pub ra_city: Option<String>,
    pub ra_province: Option<String>,
    pub ra_zip_code: Option<String>,
    pub pa_lot_no: Option<String>,
    pub pa_street: Option<String>,
    pub pa_subdivision: Option<String>,
    pub pa_barangay: Option<String>,
    pub pa_city: Option<String>,
    pub pa_province: Option<String>,
    pub pa_zip_code: Option<String>,
    pub mobile_number: Option<String>,
    pub telephone_number: Option<String>,
    pub email_address: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=personal_information)]
pub struct NewPersonalInformation {
    pub first_name: String,
    pub last_name: String,
    pub name_ext: Option<String>,
    pub middle_name: Option<String>,
    pub birth_date: String,
    pub birth_place: String,
    pub sex: String,
    pub civil_status: String,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub bloodtype: Option<String>,
    pub gsis: Option<String>,
    pub pagibig: Option<String>,
    pub philhealth: Option<String>,
    pub sss: Option<String>,
    pub tin: Option<String>,
    pub agency_employee: Option<String>,
    pub citizenship: Option<String>,
    pub ra_lot_no: Option<String>,
    pub ra_street: Option<String>,
    pub ra_subdivision: Option<String>,
    pub ra_barangay: Option<String>,
    pub ra_city: Option<String>,
    pub ra_province: Option<String>,
    pub ra_zip_code: Option<String>,
    pub pa_lot_no: Option<String>,
    pub pa_street: Option<String>,
    pub pa_subdivision: Option<String>,
    pub pa_barangay: Option<String>,
    pub pa_city: Option<String>,
    pub pa_province: Option<String>,
    pub pa_zip_code: Option<String>,
    pub mobile_number: Option<String>,
    pub telephone_number: Option<String>,
    pub email_address: Option<String>,
}