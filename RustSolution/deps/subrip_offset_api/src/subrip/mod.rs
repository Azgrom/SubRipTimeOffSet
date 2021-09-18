use std::{
    fmt::{Display, Formatter, Result as FMTResult},
    fs,
};

#[derive(Clone, Copy, PartialEq, Debug)]
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

    fn convert_units_to_ms(&self) -> u32 {
        let time_fields: Vec<u32> = [
            (self.milliseconds as u32),
            (self.seconds as u32),
            (self.minutes as u32),
            (self.hours as u32),
        ]
        .to_vec();

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

    pub fn convert_ms_to_time_info(ms_time_stamp: u32) -> Time {
        let module = |n: u32, d: u32| n - (d * (n / d) as u32);

        let milliseconds = module(ms_time_stamp, 1_000);
        let seconds = module(ms_time_stamp - milliseconds, 60_000);
        let minutes = module(
            ms_time_stamp - (seconds + milliseconds),
            3_600_000,
        );
        let hours = ms_time_stamp - (minutes + seconds + milliseconds);

        Time {
            milliseconds: milliseconds,
            seconds: (seconds / 1000) as u8,
            minutes: (minutes / 60_000) as u8,
            hours: (hours / 3_600_000) as u8,
        }
    }
}

impl TimeStamp {
    fn timestamp_splitter<'a>(timestamp_line: &'a str) -> Vec<&'a str> {
        let timestamp_indicator = " --> ";

        timestamp_line.split(timestamp_indicator).collect()
    }

    fn timestamp_parser(pattern_strings_wrapper: &Vec<&str>) -> TimeStamp {
        if pattern_strings_wrapper.len() == 2 {
            let start_end_times = TimeStamp::timestamp_splitter(pattern_strings_wrapper[1]);

            return TimeStamp {
                start: Time::time_splitter(start_end_times[0]),
                end: Time::time_splitter(start_end_times[1]),
            }
            } else {
                panic!("Undefined behavior. That should neve happen. Canceling thread.");
            }

        }

    fn subtract_fixed_offset(self, offset: u32) -> TimeStamp {
        let mut start_in_ms = self.start.convert_units_to_ms();
        let mut end_in_ms = self.end.convert_units_to_ms();

        start_in_ms = start_in_ms.saturating_sub(offset);
        end_in_ms = end_in_ms.saturating_sub(offset);

        TimeStamp {
            start: Time::convert_ms_to_time_info(start_in_ms),
            end: Time::convert_ms_to_time_info(end_in_ms),
        }
    }

    fn sum_fixed_offset(self, offset: u32) -> TimeStamp {
        let mut start_in_ms = self.start.convert_units_to_ms();
        let mut end_in_ms = self.end.convert_units_to_ms();

        start_in_ms = start_in_ms.saturating_add(offset);
        end_in_ms = end_in_ms.saturating_add(offset);

        TimeStamp {
            start: Time::convert_ms_to_time_info(start_in_ms),
            end: Time::convert_ms_to_time_info(end_in_ms),
        }
    }
}

impl SubRipFile {
    pub fn new(temp_file_path: String) -> Result<SubRipFile, &'static str> {
        if temp_file_path == "" {
            return Err("File path cannot me empty");
        }

        let temp_file_content = match fs::read_to_string(temp_file_path) {
            Ok(content_string) => content_string,
            Err(_) => String::from(
                "1
            00:02:17,440 --> 00:02:20,375
            Senator, we had a problem
            loading the legend.

            2
            00:02:20,476 --> 00:02:22,501
            God dammit, Lieutenant!
            Try uploading again!",
            ),
        };

        Ok(SubRipFile {
            filename: "Offseted_subtitle.str".to_string(),
            contents: SubRipFile::subrip_parser(&temp_file_content),
        })
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

    pub fn offset_subrip_timestamps(&mut self, mut offset: i64) {
        if offset > 0 {
            for content in self.contents.iter_mut() {
                content.dialog_timing = content
                    .dialog_timing
                    .sum_fixed_offset(offset as u32)
                    .clone();
            }
        } else {
            offset = offset.wrapping_neg();
            for content in self.contents.iter_mut() {
                content.dialog_timing = content
                    .dialog_timing
                    .subtract_fixed_offset(offset as u32)
                    .clone();
            }
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
        let result = time.convert_units_to_ms();

        assert_eq!(result, expected_result);

        let time = Time {
            hours: 7,
            minutes: 2,
            seconds: 17,
            milliseconds: 440,
        };
        let expected_result: u32 = 25_337_440;
        let result = time.convert_units_to_ms();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn milliseconds_to_time() {
        let ms_timeframe: u32 = 137_440;
        let expected_result = Time {
            hours: 0,
            minutes: 2,
            seconds: 17,
            milliseconds: 440,
        };
        let result = Time::convert_ms_to_time_info(ms_timeframe);

        assert_eq!(result, expected_result);

        let ms_timeframe: u32 = 25_337_440;
        let expected_result = Time {
            hours: 7,
            minutes: 2,
            seconds: 17,
            milliseconds: 440,
        };
        let result = Time::convert_ms_to_time_info(ms_timeframe);

        assert_eq!(result, expected_result);
    }
}
