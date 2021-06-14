#[macro_use]
extern crate rocket;

use rocket::{data::Capped, fs::TempFile};
use std::{env, io, process};
use subrip::SubRipFile;

const FILE_NAME: &str = "rocket_tmp.srt";

#[post("/file", data = "<file>")]
async fn upload(mut file: Capped<TempFile<'_>>) -> io::Result<String> {
    if file.is_complete() {
        let complete_path = format!("complete_{}", FILE_NAME);
        file.persist_to(env::temp_dir().join(complete_path)).await?;
    } else {
        let incomplete_path = format!("incomplete_{}", FILE_NAME);
        file.persist_to(env::temp_dir().join(incomplete_path))
            .await?;
    }

    let temp_file_path = file.path().unwrap().display();

    Ok(format!("{} bytes at {}", file.n.written, temp_file_path))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}
