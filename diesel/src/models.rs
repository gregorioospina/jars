use super::schema::jars;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Jar {
  pub id: i32,
  pub nickname: String,
  pub place: String,
  pub queries: i32,
  pub bar: String,
}

#[derive(Insertable)]
#[table_name = "jars"]
pub struct NewJar<'a> {
  pub nickname: &'a str,
  pub place: &'a str,
  pub queries: &'a i32,
  pub bar: &'a str,
}
