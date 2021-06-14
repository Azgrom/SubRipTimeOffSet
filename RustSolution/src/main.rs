#[macro_use] extern crate rocket;

use std::{io, env, process};
use subrip::SubRipFile;
use rocket::{data::Capped, fs::TempFile};

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

    let temp_file_path = file.path().unwrap().display();

    // println!("{} bytes at {}", file.n.written, temp_file_path);

    // let test = match SubRipFile::new(temp_file_path.to_string()) {
    //     Ok(file_content) => file_content,
    //     Err(err) => {
    //         println!("Problem parsing arguments: {}", err);
    //         process::exit(1);
    //     }
    // };
    // println!("{}", test);

    Ok(format!("{} bytes at {}", file.n.written, temp_file_path))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}
