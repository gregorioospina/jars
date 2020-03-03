#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod jar;
pub use jar::entities;

use diesel_connection::*;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "1234"
}

#[get("/jars")]
fn get_jars() -> String {
    let jars = show_jars_fn();
    for jar in &jars {}

    String::from(format!("{:?}", jars))
}

#[derive(Serialize, Deserialize)]
struct TempJar {
    name: String,
    location: String,
    bar: String,
}

#[post("/new-jar", format = "application/json", data = "<data>")]
fn new_jar(data: Json<TempJar>) -> String {
    let jar = create_jar_fn(&data.name, &data.location, &data.bar);
    println!("{:?}", jar);
    String::from("success")
}

#[get("/best-jars")]
fn show_loyal() -> String {
    let loyal = show_by_queries();
    println!("{:?}", loyal);
    String::from("gregeg")
}

use rocket::State;

#[get("/count")]
fn count(count: State<CountState>) -> String {
    let current_count = count.count;
    format!("The counter is at: {}", current_count)
}

#[get("/increase-counter/<id>")]
fn increase(count: State<CountState>, id: String) -> String {
    let jar = update_jar_fn(&id);
    String::from(format!("name, {} \nqueries, {}", jar.nickname, jar.queries))
}

struct CountState {
    count: u32,
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![count, show_loyal, get_jars, index, new_jar, increase],
        )
        .manage(CountState { count: 0 })
        .launch();
}
