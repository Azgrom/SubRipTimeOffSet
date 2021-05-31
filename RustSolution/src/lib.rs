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
    const SEC_MIN_MODULE: u16 = 60;
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

    fn match_offset(module: u16, value: &mut u16, offset: &mut u16) -> (u16, u16){
        let mut next_offset:u16 = 0;
        let module_offset = |value: u16, offset: u16| value.checked_sub(offset);

        match module_offset(module, *offset) {
            Some(i) => match module_offset(*value, *offset) {
                Some(j) => *value = j,
                None => {
                    *value += i;
                    next_offset += 1;
                }
            },
            None => {
                next_offset = *offset / module;
                *offset -= next_offset * module;
                match module_offset(*value, *offset) {
                    Some(k) => *value = k,
                    None => *value += module_offset(module, *offset).unwrap(),
                }
            }
        }

        (*value, next_offset)
    }

    fn sum_milliseconds_offset(&mut self, offset: u16) {
        if (self.milliseconds + offset) > Time::MILLISECONDS_MODULE {
            self.milliseconds += offset - Time::MILLISECONDS_MODULE;
            self.seconds += 1;
        } else {
            self.milliseconds += offset;
        }
    }

    pub fn sub_milliseconds_offset(&mut self, mut offset: u16) {

        let mut module = Time::MILLISECONDS_MODULE;
        let mut test_vec: Vec<u16> = [self.milliseconds, (self.seconds as u16), (self.minutes as u16)].to_vec();

        let mut tuple_result: (u16, u16) = (0, 0);

        for value in test_vec.iter_mut() {

            tuple_result = Time::match_offset(module, &mut *value, &mut offset);

            println!("Attributing new millisecods value: {}", tuple_result.0);
            *value = tuple_result.0;
            offset = tuple_result.1;
            module = Time::SEC_MIN_MODULE;

            if offset == 0 { break };
        }
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
