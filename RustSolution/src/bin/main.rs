use std::{env, process};
use subrip_offset_api::{SubRipFile, TimeStamp};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_file_content = SubRipFile::new(args);

    // println!("{}", parsed_file_content);

    // parsed_file_content.contents[0]
    //     .dialog_timing
    //     .start
    //     .sub_milliseconds_offset(20_589);
    // println!("{}", parsed_file_content);

    sub_test(parsed_file_content);
}

fn sub_test(subrip_file_type: SubRipFile) {
    let mut element_iterator: Option<u32> = Some(0);
    let mut mult_start_operand: u32 = 1;
    let mut mult_end_operand: u32 = 1;
    let dialog_timing: &TimeStamp = &subrip_file_type.contents[0].dialog_timing;

    let start_milli: Vec<u32> = [
        (dialog_timing.start.milliseconds as u32),
        (dialog_timing.start.seconds as u32),
        (dialog_timing.start.minutes as u32),
        (dialog_timing.start.hours as u32),
    ].to_vec();

    let end_milli: Vec<u32> = [
        (dialog_timing.end.milliseconds as u32),
        (dialog_timing.end.seconds as u32),
        (dialog_timing.end.minutes as u32),
        (dialog_timing.end.hours as u32),
    ].to_vec();

    let mut total_time_milliseconds: u32 = 0;

    for i in start_milli.iter().enumerate() {
        if *i.1 != 0 {
            match i {
                (0, y) => total_time_milliseconds += y,
                (1, y) => total_time_milliseconds += 1_000 * y,
                (2, y) => total_time_milliseconds += 60_000 * y,
                (3, y) => total_time_milliseconds += 3_600_000 * y,
                (_, _) => println!("Invalid Time field iterated element"),
            }
        }
    }

    println!("{:?}", total_time_milliseconds);
}
