use std::{fs, ops::{Add, Sub}};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct SubRipFile {
    filename: String,
    contents: Vec<SubRipContent>,
}

impl Time {
    const SEC_MIN_MODULE: u8 = 60;
    const MILLISECONDS_MODULE: u16 = 1000;

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

    fn module_offset(value: u16, diff: u16) -> Option<u16> {
        value.checked_sub(diff)
    }

    fn overflow_module_offset(f: fn(u16, u16) -> Option<u16>, diff: u16, value: u16, module: u16) {
        match f(value, diff) {
            Some(i) => value -= diff,
            None => match f(module, diff) {
                Some(i) => value += module - diff,
                None => panic!("Test")
            }
        }
    }

    fn sum_milliseconds_offset(&mut self, offset: u16) {
        if (self.milliseconds + offset) > Time::MILLISECONDS_MODULE {
            self.milliseconds += offset - Time::MILLISECONDS_MODULE;
            self.seconds += 1;
        } else {
            self.milliseconds += offset;
        }
    }

    fn sub_milliseconds_offset(&mut self, offset: u16) {
        if self.milliseconds.checked_sub(offset) == None {
            self.milliseconds += Time::MILLISECONDS_MODULE - offset;
        } else {
            self.milliseconds -= offset;
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(mut self, other: Time) -> Time {
        if self.milliseconds.checked_sub(other.milliseconds) == None {
            self.milliseconds += Time::MILLISECONDS_MODULE - other.milliseconds;
        } else {
            self.milliseconds -= other.milliseconds;
        }

        self
    }
}

impl Add for Time {
    type Output = Time;

    fn add(mut self, other: Time) -> Time {
        if self.milliseconds.checked_sub(other.milliseconds) == None {
            self.milliseconds += Time::MILLISECONDS_MODULE - other.milliseconds;
        } else {
            self.milliseconds -= other.milliseconds;
        }

        self
    }
}

impl Timestamp {
    fn timestamp_splitter<'a>(timestamp_line: &'a str) -> Vec<&'a str> {
        let timestamp_indicator = " --> ";

        timestamp_line.split(timestamp_indicator).collect()
    }

    fn timestamp_parser(pattern_strings_wrapper: &Vec<&str>) -> Timestamp {
        let start_end_times = Timestamp::timestamp_splitter(pattern_strings_wrapper[1]);

        Timestamp {
            start: Time::time_splitter(start_end_times[0]),
            end: Time::time_splitter(start_end_times[1]),
        }
    }
}

impl SubRipFile {
    pub fn new(args: Vec<String>) -> SubRipFile {
        SubRipFile {
            filename: args[1].clone(),
            contents: SubRipFile::subrip_parser(fs::read_to_string(&args[1]).unwrap().as_str()),
        }
    }

    fn subrip_dialog_parser(pattern_strings_wrapper: &Vec<&str>) -> String {
        let mut dialog_string = String::new();

        for dialog_line in &pattern_strings_wrapper[2..] {
            if *dialog_line != "" {
                dialog_string.push_str(&format!("{}\n", dialog_line));
            }
        }

        dialog_string
    }

    fn subrip_parser(subrip_textfile_content: &str) -> Vec<SubRipContent> {
        let mut pattern_strings_wrapper: Vec<&str> = Vec::new();
        let mut subrip_content_vector: Vec<SubRipContent> = Vec::new();

        for subrip_file_line in subrip_textfile_content.lines() {
            pattern_strings_wrapper.push(subrip_file_line);

            if *pattern_strings_wrapper.last().unwrap() == "" {
                let dialog_timing = Timestamp::timestamp_parser(&pattern_strings_wrapper);
                let dialog_string = SubRipFile::subrip_dialog_parser(&pattern_strings_wrapper);

                pattern_strings_wrapper.clear();

                subrip_content_vector.push(SubRipContent {
                    dialog_timing: dialog_timing,
                    dialog_string: dialog_string,
                });
            }
        }

        let dialog_timing = Timestamp::timestamp_parser(&pattern_strings_wrapper);
        let dialog_string = SubRipFile::subrip_dialog_parser(&pattern_strings_wrapper);

        subrip_content_vector.push(SubRipContent {
            dialog_timing: dialog_timing,
            dialog_string: dialog_string,
        });

        subrip_content_vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_splitter_tester() {
        let mut input = "00:01:20,439".to_string();
        let mut expected_result = (00, 01, 20, 439);
        let mut aux_var = Time::time_splitter(&input);
        let mut obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);

        input = "00:01:22,479".to_string();
        expected_result = (00, 01, 22, 479);
        aux_var = Time::time_splitter(&input);
        obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);

        input = "00:02:22,501".to_string();
        expected_result = (00, 02, 22, 501);
        aux_var = Time::time_splitter(&input);
        obtained_result = (
            aux_var.hours,
            aux_var.minutes,
            aux_var.seconds,
            aux_var.milliseconds,
        );
        assert_eq!(obtained_result, expected_result);
    }

    fn timestamp_splitter_asserter(timestamp_string: String, expected_result: Vec<&str>) {
        let obtained_result: Vec<&str> = Timestamp::timestamp_splitter(&timestamp_string);

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
    fn time_module_subtractions() {
        let mut time_example = Time {
            hours: 2,
            minutes: 17,
            seconds: 58,
            milliseconds: 584,
        };
        time_example.subtract_milliseconds_offset(10);

        let mut expected_result = Time {
            hours: 2,
            minutes: 17,
            seconds: 58,
            milliseconds: 574,
        };
        assert_eq!(time_example, expected_result);

        time_example.subtract_milliseconds_offset(580);

        expected_result = Time {
            hours: 2,
            minutes: 17,
            seconds: 58,
            milliseconds: 994,
        };
        assert_eq!(time_example, expected_result);
    }
}
