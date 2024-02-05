use std::{fmt::Display, io::Write};

use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, prelude::*, serialize::{IsNull, ToSql}};

use crate::schema::sql_types::StatusEnum;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = StatusEnum)]
pub enum Status {
    InUse,
    InStock,
    HomeOffice,
    Temporary,
    Lost,
    ToBeRepaired,
    Return,
    Sold,
    Stolen,
    AssignedLoanOffice,
    Loaned,
    Discarded,
    Other,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::InUse => write!(f, "In use"),
            Status::InStock => write!(f, "In stock"),
            Status::HomeOffice => write!(f, "Used in home office"),
            Status::Temporary => write!(f, "Temporary"),
            Status::Lost => write!(f, "Lost"),
            Status::ToBeRepaired => write!(f, "To be repaired"),
            Status::Return => write!(f, "Return supplier"),
            Status::Sold => write!(f, "Sold"),
            Status::Stolen => write!(f, "Stolen"),
            Status::AssignedLoanOffice => write!(f, "Assigned loan office"),
            Status::Loaned => write!(f, "Loaned out"),
            Status::Discarded => write!(f, "Discarded"),
            Status::Other => write!(f, "Other"),
        }
    }
}

impl ToSql<StatusEnum, Pg> for Status {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            Status::InUse => out.write_all(b"In use")?,
            Status::InStock => out.write_all(b"In stock")?,
            Status::HomeOffice => out.write_all(b"Used in home office")?,
            Status::Temporary => out.write_all(b"Temporary")?,
            Status::Lost => out.write_all(b"Lost")?,
            Status::ToBeRepaired => out.write_all(b"To be repaired")?,
            Status::Return => out.write_all(b"Return supplier")?,
            Status::Sold => out.write_all(b"Sold")?,
            Status::Stolen => out.write_all(b"Stolen")?,
            Status::AssignedLoanOffice => out.write_all(b"Assigned loan office")?,
            Status::Loaned => out.write_all(b"Loaned out")?,
            Status::Discarded => out.write_all(b"Discarded")?,
            Status::Other => out.write_all(b"Other")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<StatusEnum, Pg> for Status {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"In use" => Ok(Status::InUse),
            b"In stock" => Ok(Status::InStock),
            b"Used in home office" => Ok(Status::HomeOffice),
            b"Temporary" => Ok(Status::Temporary),
            b"Lost" => Ok(Status::Lost),
            b"To be repaired" => Ok(Status::ToBeRepaired),
            b"Return supplier" => Ok(Status::Return),
            b"Sold" => Ok(Status::Sold),
            b"Stolen" => Ok(Status::Stolen),
            b"Assigned loan office" => Ok(Status::AssignedLoanOffice),
            b"Loaned out" => Ok(Status::Loaned),
            b"Discarded" => Ok(Status::Discarded),
            b"Other" => Ok(Status::Other),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: String,
    pub serial_number: Option<String>,
    pub category: Option<String>,
    pub status: Status,
}