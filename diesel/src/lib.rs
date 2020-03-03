#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;
extern crate serde_postgres;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Jar, NewJar};

pub fn create_jar<'a>(
    conn: &PgConnection,
    nickname: &'a str,
    place: &'a str,
    queries: i32,
    bar: &'a str,
) -> Jar {
    use schema::jars;

    let new_jar = NewJar {
        nickname: &nickname.to_string(),
        place: &place.to_string(),
        queries: &queries,
        bar: &bar.to_string(),
    };

    diesel::insert_into(jars::table)
        .values(&new_jar)
        .get_result(conn)
        .expect("Error saving new jar")
}

pub fn show_jars_fn() -> std::vec::Vec<std::string::String> {
    // std::vec::Vec<>
    use schema::jars::dsl::*;

    let connection = establish_connection();
    let results = jars.load::<Jar>(&connection).expect("Error loading Jars");
    let mut list = std::vec::Vec::new();
    println!("Displaying {} jars", results.len());
    for jar in &results {
        let json = serde_json::to_string(&jar);
        list.push(json.unwrap());
    }
    println!("{:?}", list);
    list
}

pub fn create_jar_fn(name: &str, place: &str, bar: &str) -> Jar {
    let connection = establish_connection();
    let jar = create_jar(&connection, &name, &place, 0, &bar);
    println!("Saved jar! Jar id: {}", jar.id);
    jar
}

pub fn show_by_queries() -> std::vec::Vec<std::string::String> {
    use schema::jars::dsl::{jars, queries};

    let connection = establish_connection();
    let mut list = std::vec::Vec::new();
    let result = jars
        .limit(4)
        .order_by(queries.desc())
        .load::<Jar>(&connection)
        .expect("Error");

    for jar in result {
        list.push(serde_json::to_string(&jar).unwrap());
    }
    println!("most loyal {:?}", list);
    list
}

pub fn update_jar_fn(id: &str) -> Jar {
    use schema::jars::dsl::{jars, queries};

    let i_id = id.parse::<i32>().expect("Invalid ID");

    let connection = establish_connection();
    let jar = diesel::update(jars.find(i_id))
        .set(queries.eq(queries + 1))
        .get_result::<Jar>(&connection)
        .expect(&format!("Unable to find {}", id));
    println!("Updated jar {}, {}", jar.nickname, jar.queries);
    jar
}
