extern crate diesel;
extern crate diesel_connection;

use self::diesel::prelude::*;
use self::diesel_connection::*;
use self::models::Jar;
use std::env::args;

fn main() {
  use diesel_connection::schema::jars::dsl::{jars, queries};

  let id = args()
    .nth(1)
    .expect("You must enter a valid ID")
    .parse::<i32>()
    .expect("Invalid ID");
  let connection = establish_connection();

  let jar = diesel::update(jars.find(id))
    .set(queries.eq(queries + 1))
    .get_result::<Jar>(&connection)
    .expect(&format!("Unable to find {}", id));
  println!("Updated jar {}, {}", jar.nickname, jar.queries)
}
