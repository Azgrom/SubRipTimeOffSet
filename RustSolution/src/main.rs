#[macro_use]
extern crate rocket;
use subrip_web_api::{file, upload};

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 1111));

    rocket::custom(figment).mount("/", routes![upload, file])
}
