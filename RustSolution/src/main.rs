#[macro_use]
extern crate rocket;
use subrip_web_api::{file, upload};

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 5000));

    rocket::custom(figment).mount("/", routes![upload, file])
}
