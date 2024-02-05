use diesel::prelude::*;

use crate::{schema::*, Status};

#[derive(Queryable, Selectable)]
#[diesel(table_name = devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: String,
    pub serial_number: Option<String>,
    pub category: Option<String>,
    pub status: Status,
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice<'a> {
    pub id: &'a str,
    pub serial_number: Option<&'a str>,
}
