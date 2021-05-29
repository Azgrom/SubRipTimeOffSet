use std::fs;

#[derive(Debug, PartialEq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

#[derive(Debug)]
pub struct Timestamp {
    pub start: Time,
    pub end: Time,
}

#[derive(Debug)]
pub struct SubRipContent {
    pub dialog_timing: Timestamp,
    pub dialog_string: String,
}

#[derive(Debug)]
pub struct SubRipFile {
    filename: String,
    pub contents: Vec<SubRipContent>,
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

    fn overflow_module_offset(&mut self, f: fn(u16, u16) -> Option<u16>, mut diff: u16) {
        let time_modules = (Time::MILLISECONDS_MODULE, Time::SEC_MIN_MODULE);
        let mut value = self.milliseconds;
        let module = time_modules.0;
        let mut sec_offset: u16 = 0;

        match f(module, diff) {
            Some(i) => match f(value, diff) {
                Some(j) => value = j,
                None => {
                    value += i;
                    sec_offset += 1;
                }
            },
            None => {
                sec_offset = diff / module;
                diff -= sec_offset * module;
                match f(value, diff) {
                    Some(k) => value = k,
                    None => value += f(module, diff).unwrap(),
                }
            }
        }

        println!("Attributing new millisecods value: {}", value);
        self.milliseconds = value;

        let mut value = self.seconds as u16;
        let module = time_modules.1 as u16;
        let mut min_offset: u16 = 0;

        match f(module, sec_offset) {
            Some(i) => match f(value, sec_offset) {
                Some(j) => value = j,
                None => {
                    value += i;
                    min_offset += 1;
                }
            },
            None => {
                min_offset = sec_offset / module;
                sec_offset -= min_offset * module;
                match f(value, sec_offset) {
                    Some(k) => value = k,
                    None => value += f(module, diff).unwrap(),
                }
            }
        }

        println!("Attributing new seconds value: {}", value);
        self.seconds = value as u8;

        let mut value = self.minutes as u16;
        let module = time_modules.1 as u16;
        let mut hour_offset: u16 = 0;

        match f(module, min_offset) {
            Some(i) => match f(value, min_offset) {
                Some(j) => value = j,
                None => {
                    value += i;
                    hour_offset += 1;
                }
            },
            None => {
                min_offset = min_offset / module;
                min_offset -= min_offset * module;
                match f(value, min_offset) {
                    Some(k) => value = k,
                    None => value += f(module, diff).unwrap(),
                }
            }
        }

        println!("Attributing new minutes value: {}", value);
        self.minutes = value as u8;
    }

    fn sum_milliseconds_offset(&mut self, offset: u16) {
        if (self.milliseconds + offset) > Time::MILLISECONDS_MODULE {
            self.milliseconds += offset - Time::MILLISECONDS_MODULE;
            self.seconds += 1;
        } else {
            self.milliseconds += offset;
        }
    }

    pub fn sub_milliseconds_offset(&mut self, offset: u16) {
        self.overflow_module_offset(Time::module_offset, offset);
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
        time_example.sub_milliseconds_offset(10);

        assert_eq!(
            time_example,
            Time {
                hours: 2,
                minutes: 17,
                seconds: 58,
                milliseconds: 574,
            }
        );

        time_example.sub_milliseconds_offset(2000);

        println!("{:?}", time_example);

        assert_eq!(
            time_example,
            Time {
                hours: 2,
                minutes: 17,
                seconds: 58,
                milliseconds: 994,
            }
        );
    }
}
