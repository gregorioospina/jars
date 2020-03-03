extern crate diesel;
extern crate diesel_connection;

use self::diesel_connection::*;
use std::io::{stdin, Read};

fn main() {
  let connection = establish_connection();

  println!("What would you like your name to be?");
  let mut nickname = String::new();
  stdin().read_line(&mut nickname).unwrap();
  let nickname = &nickname[..(nickname.len() - 1)];
  println!("\nWhere will the Jar be stored?");
  let mut place = String::new();
  stdin().read_line(&mut place).unwrap();
  let place = &place[..(place.len() - 1)];
  println!("\nIn what bar?");
  let mut bar = String::new();
  stdin().read_line(&mut bar).unwrap();
  let bar = &bar[..(bar.len() - 1)];

  let jar = create_jar(&connection, nickname, place, 0, bar);
  println!("Saved jar! Jar id: {}", jar.id);
}
