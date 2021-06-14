#[macro_use] extern crate rocket;

use std::{io, env};
use rocket::data::Capped;
use rocket::fs::TempFile;

const FILE_NAME: &str = "rocket_tmp.srt";


#[post("/file", data = "<file>")]
async fn upload(mut file: Capped<TempFile<'_>>) -> io::Result<String> {

    if file.is_complete() {
        let complete_path = format!("rocket-app/complete/{}", FILE_NAME);
        file.persist_to(env::temp_dir().join(complete_path)).await?;
    } else {
        let incomplete_path = format!("rocket-app/incomplete/{}", FILE_NAME);
        file.persist_to(env::temp_dir().join(incomplete_path)).await?;
    }

    Ok(format!("{} bytes at {}", file.n.written, file.path().unwrap().display()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}
