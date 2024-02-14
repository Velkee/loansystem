use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{schema::*, Status};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(primary_key(code))]
#[diesel(table_name = post_offices)]
pub struct PostOffice {
    pub code: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = post_offices)]
pub struct NewPostOffice<'a> {
    pub code: &'a i32,
    pub name: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(primary_key(name))]
#[diesel(belongs_to(PostOffice, foreign_key = post_office))]
#[diesel(table_name = locations)]
pub struct Location {
    pub name: String,
    pub address: Option<String>,
    pub post_office: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = locations)]
pub struct NewLocation<'a> {
    pub name: &'a str,
    pub address: Option<&'a str>,
    pub post_office: Option<&'a i32>,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(primary_key(name))]
#[diesel(belongs_to(Location, foreign_key = location))]
#[diesel(table_name = departments)]
pub struct Department {
    pub name: String,
    pub location: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = departments)]
pub struct NewDepartment<'a> {
    pub name: &'a str,
    pub location: Option<&'a str>,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(primary_key(name))]
#[diesel(table_name = buildings)]
pub struct Building {
    pub name: String,
    pub address: Option<String>,
    pub location: Option<String>,
    pub post_office: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = buildings)]
pub struct NewBuilding<'a> {
    pub name: &'a str,
    pub address: Option<&'a str>,
    pub location: Option<&'a str>,
    pub post_office: Option<&'a i32>,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(primary_key(name))]
#[diesel(belongs_to(Building, foreign_key = building))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub name: String,
    pub building: String,
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub building: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(Location, foreign_key = location))]
#[diesel(belongs_to(Department, foreign_key = department))]
#[diesel(belongs_to(Building, foreign_key = building))]
#[diesel(table_name = people)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub department: Option<String>,
    pub building: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = people)]
pub struct NewPerson<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub location: Option<&'a str>,
    pub department: Option<&'a str>,
    pub building: Option<&'a str>,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Category, foreign_key = category))]
#[diesel(belongs_to(Person, foreign_key = person))]
#[diesel(belongs_to(Location, foreign_key = location))]
#[diesel(belongs_to(Department, foreign_key = department))]
#[diesel(belongs_to(Building, foreign_key = building))]
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
    pub person: Option<&'a str>,
    pub location: Option<&'a str>,
    pub department: Option<&'a str>,
    pub building: Option<&'a str>,
    pub room: Option<&'a str>,
}
