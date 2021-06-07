use std::{env, process};
use subrip_offset_api::SubRipFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_file_content = match SubRipFile::new(args) {
        Ok(parsed_file_content) => parsed_file_content,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    println!("{}", parsed_file_content);

    parsed_file_content.offset_subrip_timestamps(20_589);
    println!("{}", parsed_file_content);
}
