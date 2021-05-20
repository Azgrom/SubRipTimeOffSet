use std::{env, fs, process};

#[derive(Debug)]
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

struct Timestamp {
    start: Time,
    end: Time,
}

struct SubRipFile {
    filename: String,
    contents: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = SubRipFile {
        filename: args[1].clone(),
        contents: match fs::read_to_string(&args[1]) {
            Ok(contents) => contents,
            Err(_err) => {
                println!("Fucking problems...");
                process::exit(1);
            }
        },
    };

    let example = file.contents.lines();

    println!("{:?}", file.filename);
    for line in example {
        if line.len() == 29 && line.contains(" --> ") {
            for timestamp in timestamp_splitter(line) {
                println!("{:?}", time_splitter(timestamp));
            }
        }
    }
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
