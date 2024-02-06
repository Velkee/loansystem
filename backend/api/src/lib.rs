pub mod models;
pub mod schema;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, ToSql};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;
use schema::sql_types::StatusEnum;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;
use std::io::Write;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Deserialize, Serialize)]
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
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
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
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
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
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
