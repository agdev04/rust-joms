
use diesel::ExpressionMethods;
use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
// use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub struct PersonalInformationRepo;

impl PersonalInformationRepo {
    pub async fn all(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<PersonalInformation>> {
        personal_information::table.limit(limit).get_results(c).await
    }

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<PersonalInformation> {
        personal_information::table.find(id).get_result(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_personal_information: NewPersonalInformation) -> QueryResult<PersonalInformation> {
        diesel::insert_into(personal_information::table)
        .values(new_personal_information)
        .get_result(c)
        .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, personal_information: PersonalInformation) -> QueryResult<PersonalInformation> {
        diesel::update(personal_information::table.find(id))
        .set((
            personal_information::first_name.eq(personal_information.first_name),
            personal_information::last_name.eq(personal_information.last_name),
            personal_information::name_ext.eq(personal_information.name_ext),
            personal_information::middle_name.eq(personal_information.middle_name),
            personal_information::birth_date.eq(personal_information.birth_date),
            personal_information::birth_place.eq(personal_information.birth_place),
            personal_information::sex.eq(personal_information.sex),
            personal_information::civil_status.eq(personal_information.civil_status),
            personal_information::height.eq(personal_information.height),
            personal_information::weight.eq(personal_information.weight),
            personal_information::bloodtype.eq(personal_information.bloodtype),
            personal_information::gsis.eq(personal_information.gsis),
            personal_information::pagibig.eq(personal_information.pagibig),
            personal_information::philhealth.eq(personal_information.philhealth),
            personal_information::sss.eq(personal_information.sss),
            personal_information::tin.eq(personal_information.tin),
            personal_information::agency_employee.eq(personal_information.agency_employee),
            personal_information::citizenship.eq(personal_information.citizenship),
            personal_information::ra_lot_no.eq(personal_information.ra_lot_no),
            personal_information::ra_street.eq(personal_information.ra_street),
            personal_information::ra_subdivision.eq(personal_information.ra_subdivision),
            personal_information::ra_barangay.eq(personal_information.ra_barangay),
            personal_information::ra_city.eq(personal_information.ra_city),
            personal_information::ra_province.eq(personal_information.ra_province),
            personal_information::ra_zip_code.eq(personal_information.ra_zip_code),
            personal_information::pa_lot_no.eq(personal_information.pa_lot_no),
            personal_information::pa_street.eq(personal_information.pa_street),
            personal_information::pa_subdivision.eq(personal_information.pa_subdivision),
            personal_information::pa_barangay.eq(personal_information.pa_barangay),
            personal_information::pa_city.eq(personal_information.pa_city),
            personal_information::pa_province.eq(personal_information.pa_province),
            personal_information::pa_zip_code.eq(personal_information.pa_zip_code),
            personal_information::mobile_number.eq(personal_information.mobile_number),
            personal_information::telephone_number.eq(personal_information.telephone_number),
            personal_information::email_address.eq(personal_information.email_address),
        ))
        .get_result(c)
        .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(personal_information::table.find(id)).execute(c).await
    }
}
