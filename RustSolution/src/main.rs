use std::{env, fs, process};

#[derive(Debug)]
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug)]
struct Timestamp {
    start: Time,
    end: Time,
}

#[derive(Debug)]
struct SubRipContent {
    dialog_timing: Timestamp,
    dialog_string: String,
}

struct SubRipFile {
    filename: String,
    contents: Vec<SubRipContent>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args[1]);
    println!("{:?}", subrip_parser(fs::read_to_string(&args[1]).unwrap().as_str()));
}

fn timestamp_splitter<'a>(timestamp_line: &'a str) -> Vec<&'a str> {
    let timestamp_indicator = " --> ";

    timestamp_line.split(timestamp_indicator).collect()
}

fn time_splitter<'a>(time_str: &'a str) -> Time {
    let split_parameter = [':', ','];

    let time_vec = time_str.split(&split_parameter[..]).collect::<Vec<&str>>();

    Time {
        hours: time_vec[0].parse::<u8>().unwrap(),
        minutes: time_vec[1].parse::<u8>().unwrap(),
        seconds: time_vec[2].parse::<u8>().unwrap(),
        milliseconds: time_vec[3].parse::<u16>().unwrap(),
    }
}

fn subrip_sintax_pattern_identifier<'a>(subrip_textfile_content: &'a str) -> Vec<&'a str>{
    let mut subrip_textfile_content = subrip_textfile_content.lines();
    let mut pattern_strings_wrapper = Vec::new();

    loop {
        let subrip_file_line = subrip_textfile_content.next().unwrap();
        pattern_strings_wrapper.push(subrip_file_line);
        if subrip_file_line == "" {
            break;
        }
    }

    pattern_strings_wrapper
}

fn subrip_parser(example: &str) -> SubRipContent {

    let v = subrip_sintax_pattern_identifier(example);

    let start_end_times = timestamp_splitter(v[1]);
    let dialog_timing = Timestamp {
        start: time_splitter(start_end_times[0]),
        end: time_splitter(start_end_times[1]),
    };

    let mut dialog_string = String::new();

    for dialog_line in &v[2..] {
        if *dialog_line != "" {
            dialog_string.push_str(&format!("{}\n", dialog_line));
        }
    }

    SubRipContent {
        dialog_timing: dialog_timing,
        dialog_string: dialog_string,
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp_splitter_asserter(timestamp_string: String, expected_result: Vec<&str>) {
        let obtained_result: Vec<&str> = timestamp_splitter(&timestamp_string);

        assert_eq!(expected_result, obtained_result);
    }

    #[test]
    fn timestamp_splitter_tester() {
        let timestamp_string: String = "00:02:17,440 --> 00:02:20,375".to_string();
        let expected_result: Vec<&str> = ["00:02:17,440", "00:02:20,375"].to_vec();
        timestamp_splitter_asserter(timestamp_string, expected_result);

        let timestamp_string: String = "00:02:20,476 --> 00:02:22,501".to_string();
        let expected_result: Vec<&str> = ["00:02:20,476", "00:02:22,501"].to_vec();
        timestamp_splitter_asserter(timestamp_string, expected_result);

        let timestamp_string: String = "00:01:20,439 --> 00:01:22,479".to_string();
        let expected_result: Vec<&str> = ["00:01:20,439", "00:01:22,479"].to_vec();
        timestamp_splitter_asserter(timestamp_string, expected_result);
    }

    #[test]
    fn time_splitter_tester() {
        let input = "00:01:20,439".to_string();
        let expected_result = (00, 01, 20, 439);
        let aux_var = time_splitter(&input);
        let obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);

        let input = "00:01:22,479".to_string();
        let expected_result = (00, 01, 22, 479);
        let aux_var = time_splitter(&input);
        let obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);

        let input = "00:02:22,501".to_string();
        let expected_result = (00, 02, 22, 501);
        let aux_var = time_splitter(&input);
        let obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);
    }
}
