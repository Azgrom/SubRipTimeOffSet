#[macro_use]
extern crate rocket;

use rocket::{data::Capped, fs::{NamedFile, TempFile}};
use std::{env, io, process};
use subrip::SubRipFile;

const FILE_NAME: &str = "rocket_tmp.srt";

#[post("/file", data = "<file>")]
async fn upload(mut file: Capped<TempFile<'_>>) -> io::Result<String> {
    file.persist_to(env::temp_dir().join(FILE_NAME)).await?;
    Ok(format!("{} bytes at {}", file.n.written, file.path().unwrap().display()))
}

#[get("/file")]
async fn file() -> Option<NamedFile> {
    NamedFile::open(env::temp_dir().join(FILE_NAME)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload, file])
}
