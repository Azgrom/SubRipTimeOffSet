#[macro_use]
extern crate rocket;

use rocket::{
    data::Capped,
    fs::{NamedFile, TempFile},
};
use std::{env, io, process};
use subrip::SubRipFile;

const FILE_NAME: &str = "rocket_tmp.srt";

#[post("/file", data = "<file>")]
pub async fn upload(mut file: Capped<TempFile<'_>>) -> io::Result<String> {
    file.persist_to(env::temp_dir().join(FILE_NAME)).await?;
    Ok(format!(
        "{} bytes at {}",
        file.n.written,
        file.path().unwrap().display()
    ))
}

#[get("/file/offset/<n>")]
pub async fn file(n: f64) -> Option<NamedFile> {
    let file_path: String = env::temp_dir()
        .join(FILE_NAME)
        .into_os_string()
        .into_string()
        .unwrap();
    let mut subrip_content = match SubRipFile::new(file_path.as_str().to_string()) {
        Ok(file_content) => file_content,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    subrip_content.offset_subrip_timestamps((n * 1000.0) as i64);
    match subrip_content.export_to_file(file_path) {
        Ok(()) => (),
        Err(err) => println!("\t\t{}", err),
    }
    NamedFile::open(env::temp_dir().join(FILE_NAME)).await.ok()
}
