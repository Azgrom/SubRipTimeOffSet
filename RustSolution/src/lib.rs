pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;

use self::models::{NewPost, Post};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) -> usize {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

mod Crud {
    use super::*;
    
    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";

    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";

    pub fn show_posts() {
        use diesel_demo::schema::posts::dsl::*;

        let connection = establish_connection();
        let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

        println!("Displaying {} posts", results.len());

        for post in results {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }
    }

    pub fn delete_post() {
        use diesel_demo::schema::posts::dsl::*;

        let target = env::args().nth(1).expect("Expected a targed to match against");
        let pattern = format!("%{}%", target);

        let connection = establish_connection();
        let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

        println!("Deleted {} posts", num_deleted);
    }
}
