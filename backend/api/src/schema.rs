// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_enum"))]
    pub struct StatusEnum;
}

diesel::table! {
    buildings (name) {
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        post_office -> Nullable<Int4>,
    }
}

diesel::table! {
    categories (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    departments (name) {
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        location -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    devices (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        serial_number -> Nullable<Varchar>,
        #[max_length = 255]
        category -> Nullable<Varchar>,
        status -> StatusEnum,
        #[max_length = 255]
        person -> Nullable<Varchar>,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        #[max_length = 255]
        department -> Nullable<Varchar>,
        #[max_length = 255]
        building -> Nullable<Varchar>,
        #[max_length = 255]
        room -> Nullable<Varchar>,
    }
}

diesel::table! {
    locations (name) {
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        post_office -> Nullable<Int4>,
    }
}

diesel::table! {
    people (id) {
        #[max_length = 255]
        id -> Varchar,
        name -> Text,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        #[max_length = 255]
        department -> Nullable<Varchar>,
        #[max_length = 255]
        building -> Nullable<Varchar>,
    }
}

diesel::table! {
    post_offices (code) {
        code -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    rooms (name) {
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        building -> Varchar,
    }
}

diesel::joinable!(buildings -> locations (location));
diesel::joinable!(buildings -> post_offices (post_office));
diesel::joinable!(departments -> locations (location));
diesel::joinable!(devices -> buildings (building));
diesel::joinable!(devices -> categories (category));
diesel::joinable!(devices -> departments (department));
diesel::joinable!(devices -> locations (location));
diesel::joinable!(devices -> people (person));
diesel::joinable!(devices -> rooms (room));
diesel::joinable!(locations -> post_offices (post_office));
diesel::joinable!(people -> buildings (building));
diesel::joinable!(people -> departments (department));
diesel::joinable!(people -> locations (location));
diesel::joinable!(rooms -> buildings (building));

diesel::allow_tables_to_appear_in_same_query!(
    buildings,
    categories,
    departments,
    devices,
    locations,
    people,
    post_offices,
    rooms,
);
