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

fn create_post<'a>(conn: &SqliteConnection, post_title: &'a str, post_body: &'a str) -> usize {
    use schema::posts;

    let new_post = NewPost {
        title: post_title,
        body: post_body,
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

    pub fn write_post() {
        let connection = establish_connection();

        println!("What would you like your title to be?");
        let mut post_title = String::new();
        stdin().read_line(&mut post_title).unwrap();

        let post_title = &post_title[..(post_title.len() - 1)]; // Drop the newline character
        println!(
            "\nOk! Let's write '{}' (Press {} when finished)\n",
            post_title, EOF
        );

        let mut post_body = String::new();
        stdin().read_to_string(&mut post_body).unwrap();

        let post = create_post(&connection, post_title, &post_body);
        println!("\nSaved draft '{}' with id {}", post_title, post);
    }

    pub fn delete_post() {
        let target = env::args().nth(1).expect("Expected a targed to match against");
        let pattern = format!("%{}%", target);

        let connection = establish_connection();
        let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

        println!("Deleted {} posts", num_deleted);
    }

    pub fn publish_post() {
        let post_id = env::args()
            .nth(1)
            .expect("publish_post requires a post id")
            .parse::<i32>()
            .expect("Invalid ID");
        let connection = establish_connection();

        let _ = diesel::update(posts.find(post_id))
            .set(published.eq(true))
            .execute(&connection)
            .unwrap_or_else(|_| panic!("Unable to find post {}", post_id));

        let post: Post = posts
            .find(post_id)
            .first(&connection)
            .unwrap_or_else(|_| panic!("Unable to find post {}", post_id));

        println!("Published post {}", post.title);
    }
}
