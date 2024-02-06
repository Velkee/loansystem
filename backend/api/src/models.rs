use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{schema::*, Status};

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(Person, foreign_key = id))]
#[diesel(table_name = devices)]
pub struct Device {
    pub id: String,
    pub serial_number: Option<String>,
    pub category: Option<String>,
    pub status: Status,
    pub person: Option<String>,
    pub location: Option<String>,
    pub department: Option<String>,
    pub building: Option<String>,
    pub room: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice<'a> {
    pub id: &'a str,
    pub serial_number: Option<&'a str>,
    pub category: Option<&'a str>,
    pub status: &'a Status,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = people)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub department: Option<String>,
    pub building: Option<String>,
}
