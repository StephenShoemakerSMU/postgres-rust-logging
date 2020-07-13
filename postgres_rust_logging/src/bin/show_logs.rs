extern crate postgres_rust_logging;
extern crate diesel;

use self::postgres_rust_logging::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use postgres_rust_logging::schema::logs::dsl::*;

    let connection = establish_connection();
    let results = logs.filter(resolved.eq(true))
        .limit(5)
        .load::<Log>(&connection)
        .expect("Error loading logs");

    println!("Displaying {} logs", results.len());
    for log in results {
        println!("{}", log.log_id);
    }
}