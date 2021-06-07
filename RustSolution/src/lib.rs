use std::{
    fmt::{Display, Formatter, Result as FMTResult},
    fs
};

#[derive(Clone, Copy)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u32,
}

#[derive(Clone, Copy)]
pub struct TimeStamp {
    pub start: Time,
    pub end: Time,
}

pub struct SubRipContent {
    pub dialog_timing: TimeStamp,
    pub dialog_string: String,
}

pub struct SubRipFile {
    pub filename: String,
    pub contents: Vec<SubRipContent>,
}

impl Time {
    const SEC_MIN_MODULE: u32 = 60;
    const MILLISECONDS_MODULE: u32 = 1000;

    fn time_splitter<'a>(time_str: &'a str) -> Time {
        let split_parameter = [':', ','];

        let time_vec = time_str.split(&split_parameter[..]).collect::<Vec<&str>>();

        Time {
            hours: time_vec[0].parse::<u8>().unwrap(),
            minutes: time_vec[1].parse::<u8>().unwrap(),
            seconds: time_vec[2].parse::<u8>().unwrap(),
            milliseconds: time_vec[3].parse::<u32>().unwrap(),
        }
    }

    fn convert_units_to_milliseconds(&self) -> u32 {
        let time_fields: Vec<u32> = [
            (self.milliseconds as u32),
            (self.seconds as u32),
            (self.minutes as u32),
            (self.hours as u32),
        ].to_vec();

        let mut total_time_milliseconds: u32 = 0;

        for i in time_fields.iter().enumerate() {
            if *i.1 != 0 {
                match i {
                    (0, y) => total_time_milliseconds += y,
                    (1, y) => total_time_milliseconds += y * 1_000,
                    (2, y) => total_time_milliseconds += y * 60_000,
                    (3, y) => total_time_milliseconds += y * 3_600_000,
                    (_, _) => println!("Invalid Time field iterated element"),
                }
            }
        }

        total_time_milliseconds
    }

    fn convert_milliseconds_to_time_units(milliseconds_time_stamp: u32) -> Time {
        let milliseconds = milliseconds_time_stamp % 1_000;
        let seconds = (((milliseconds_time_stamp - (milliseconds as u32)) % 60_000) / 1_000) as u8;
        let minutes = (((milliseconds_time_stamp - (((seconds as u32) * 1_000) + (milliseconds as u32))) % 3_600_000) / 1000) as u8;
        let hours = (milliseconds_time_stamp - ((minutes as u32) + (seconds as u32) + (milliseconds as u32))) as u8;

        Time {
            milliseconds: milliseconds,
            seconds: seconds,
            minutes: minutes,
            hours: hours,
        }
    }
}

impl TimeStamp {
    fn timestamp_splitter<'a>(timestamp_line: &'a str) -> Vec<&'a str> {
        let timestamp_indicator = " --> ";

        timestamp_line.split(timestamp_indicator).collect()
    }

    fn timestamp_parser(pattern_strings_wrapper: &Vec<&str>) -> TimeStamp {
        let start_end_times = TimeStamp::timestamp_splitter(pattern_strings_wrapper[1]);

        TimeStamp {
            start: Time::time_splitter(start_end_times[0]),
            end: Time::time_splitter(start_end_times[1]),
        }
    }

    fn subtract_fixed_offset(self, offset: u32) -> TimeStamp {
        let mut start_in_milliseconds = self.start.convert_units_to_milliseconds();
        let mut end_in_milliseconds = self.end.convert_units_to_milliseconds();

        start_in_milliseconds -= offset;
        end_in_milliseconds -= offset;

        TimeStamp {
            start: Time::convert_milliseconds_to_Time_units(start_in_milliseconds),
            end: Time::convert_milliseconds_to_Time_units(end_in_milliseconds),
        }
    }

    fn sum_fixed_offset(self, offset: u32) -> TimeStamp {
        let mut start_in_milliseconds = self.start.convert_units_to_milliseconds();
        let mut end_in_milliseconds = self.end.convert_units_to_milliseconds();

        start_in_milliseconds += offset;
        end_in_milliseconds += offset;

        TimeStamp {
            start: Time::convert_milliseconds_to_Time_units(start_in_milliseconds),
            end: Time::convert_milliseconds_to_Time_units(end_in_milliseconds),
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
                let dialog_timing = TimeStamp::timestamp_parser(&pattern_strings_wrapper);
                let dialog_string = SubRipFile::subrip_dialog_parser(&pattern_strings_wrapper);

                pattern_strings_wrapper.clear();

                subrip_content_vector.push(SubRipContent {
                    dialog_timing: dialog_timing,
                    dialog_string: dialog_string,
                });
            }
        }

        let dialog_timing = TimeStamp::timestamp_parser(&pattern_strings_wrapper);
        let dialog_string = SubRipFile::subrip_dialog_parser(&pattern_strings_wrapper);

        subrip_content_vector.push(SubRipContent {
            dialog_timing: dialog_timing,
            dialog_string: dialog_string,
        });

        subrip_content_vector
    }

    pub fn offset_subrip_timestamps(&mut self, offset: u32) {
        for content in self.contents.iter_mut() {
            content.dialog_timing = content.dialog_timing.subtract_fixed_offset(offset).clone();
        }
    }
}

mod stdout_implementation {
    use super::*;

    impl Display for Time {
        fn fmt(&self, f: &mut Formatter) -> FMTResult {
            write!(
                f,
                "{:02}:{:02}:{:02},{:03}",
                self.hours, self.minutes, self.seconds, self.milliseconds
            )
        }
    }

    impl Display for TimeStamp {
        fn fmt(&self, f: &mut Formatter) -> FMTResult {
            write!(f, "{} --> {}", self.start, self.end)
        }
    }

    impl Display for SubRipContent {
        fn fmt(&self, f: &mut Formatter) -> FMTResult {
            write!(f, "{}\n{}\n", self.dialog_timing, self.dialog_string)
        }
    }

    impl Display for SubRipFile {
        fn fmt(&self, f: &mut Formatter) -> FMTResult {
            let mut content_string = String::new();

            for content in self.contents.iter().enumerate() {
                content_string.push_str(&(content.0 + 1).to_string());
                content_string.push('\n');
                content_string.push_str(&content.1.to_string());
            }

            write!(
                f,
                "Filename: {}\nNumber of dialogs: {}\n\n\nDialogs:\n----------\n{}",
                self.filename,
                self.contents.len(),
                content_string
            )
        }
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
        let obtained_result: Vec<&str> = TimeStamp::timestamp_splitter(&timestamp_string);

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
    fn time_to_milliseconds() {
        let time = Time {
            hours: 0,
            minutes: 2,
            seconds: 17,
            milliseconds: 440,
        };

        let expected_result: u32 = 137_440;
        let result = time.convert_units_to_milliseconds();

        assert_eq!(result, expected_result);
    }
}
