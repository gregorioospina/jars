extern crate diesel;
extern crate diesel_connection;

use self::diesel::prelude::*;
use self::diesel_connection::*;
use self::models::*;

fn main() {
  use diesel_connection::schema::jars::dsl::*;

  let connection = establish_connection();
  let results = jars
    .limit(5)
    .load::<Jar>(&connection)
    .expect("Error loading Jars");

  println!("Displaying {} jars", results.len());
  for jar in results {
    println!(" Name: {},", jar.nickname);
    println!(" Location: {},", jar.place);
    println!(" Bar: {},", jar.bar);
    println!(" Number of queries: {},", jar.queries);
  }
  results
}
