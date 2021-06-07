use std::{env, process};
use subrip_offset_api::{SubRipFile, TimeStamp};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_file_content = SubRipFile::new(args);

    println!("{}", parsed_file_content);

    parsed_file_content.contents[0]
        .dialog_timing
        .start
        .sub_milliseconds_offset(20_589);
    println!("{}", parsed_file_content);
}
