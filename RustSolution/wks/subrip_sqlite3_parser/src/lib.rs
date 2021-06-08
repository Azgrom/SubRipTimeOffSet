pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;

use self::{
    models::{NewPSubRipRegistry, SubRipRegistry},
    schema::subrip_reg::dsl::*,
};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &SqliteConnection, reg_filename: &'a str, reg_content: &'a str) -> usize {
    use schema::subrip_reg;

    let new_post = NewPSubRipRegistry {
        filename: reg_filename,
        content: reg_content,
    };

    diesel::insert_into(subrip_reg::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub mod crud {
    use super::*;
    use std::io::{stdin, Read};

    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";

    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";

    pub fn show_regs() {
        let connection = establish_connection();
        let results = subrip_reg
            .filter(published.eq(true))
            .limit(5)
            .load::<SubRipRegistry>(&connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());

        for reg in results {
            println!("{}", reg.filename);
            println!("----------\n");
            println!("{}", reg.content);
        }
    }

    pub fn write_reg(reg_filename: &str, reg_content: String) {
        let connection = establish_connection();

        let reg_filename = &reg_filename[..(reg_filename.len() - 1)]; // Drop the newline character

        let post = create_post(&connection, reg_filename, &reg_content);
        println!("\nSaved draft '{}' with id {}", reg_filename, post);
    }

    pub fn delete_reg() {
        let target = env::args()
            .nth(1)
            .expect("Expected a targed to match against");
        let pattern = format!("%{}%", target);

        let connection = establish_connection();
        let num_deleted = diesel::delete(subrip_reg.filter(filename.like(pattern)))
            .execute(&connection)
            .expect("Error deleting posts");

        println!("Deleted {} posts", num_deleted);
    }

    pub fn publish_reg() {
        let reg_id: i32 = 1;
        let connection = establish_connection();

        let _ = diesel::update(subrip_reg.find(reg_id))
            .set(published.eq(true))
            .execute(&connection)
            .unwrap_or_else(|_| panic!("Unable to find post {}", reg_id));

        let post: SubRipRegistry = subrip_reg
            .find(reg_id)
            .first(&connection)
            .unwrap_or_else(|_| panic!("Unable to find post {}", reg_id));

        println!("Published post {}", post.filename);
    }
}
