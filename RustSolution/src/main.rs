#[macro_use]
extern crate rocket;
use subrip_web_api::{upload, file};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload, file])
}
