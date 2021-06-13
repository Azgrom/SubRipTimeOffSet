#[macro_use] extern crate rocket;

use std::io;
use rocket::data::{Data, ToByteUnit};

#[post("/", data = "<data>")]
async fn data_guard(mut data: Data<'_>) -> io::Result<Vec<u8>> {
    println!("All of the data: {:?}", data.peek(512).await);

    let bytes = data.open(200.kibibytes()).into_bytes().await?;
    if !bytes.is_complete() {
        println!("there are bytes remaining in the stream");
    }

    // println!("{:?}", std::str::from_utf8(&bytes.value));

    Ok(bytes.into_inner())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![data_guard])
}
