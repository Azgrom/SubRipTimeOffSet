use std::{env, process};
use subrip_offset_api::{SubRipFile, Time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_file_content = SubRipFile::new(args);

    println!("{:?}", parsed_file_content);
}
