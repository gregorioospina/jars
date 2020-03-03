// extern crate diesel;
// extern crate diesel_connection;

// use self::diesel::prelude::*;
// use self::diesel_connection::*;
// use self::models::*;
// use diesel::debug_query;
// use std::env::args;

// fn main() {
//   use diesel_connection::schema::jars::dsl::*;
//   let name = args().nth(1).expect("Please give a valid name");
//   let pattern = format!("%{}%", name);
//   println!("{}", name);
//   let connection = establish_connection();
//   let busqueda = jars
//     .filter(nickname.eq(name))
//     .load::<Jar>(&connection)
//     .expect("No se pudo encontrar el jarro");
//   let debug = debug_query::<jars, _>(&busqueda);

//   println!("{:#?}", busqueda);
//   println!("{}", debug);

//   for jar in busqueda {
//     println!("{}", jar.nickname);
//     println!("{}", jar.place);
//   }
// }
