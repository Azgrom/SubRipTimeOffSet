use std::{env, fs, process};

struct SubRipFile {
    filename: String,
    contents: String,
}

pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
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

    println!("{:?}", file.contents);
    for line in example {
        if line.len() == 29 && line.contains(" --> ") {
            println!("{:?}", timestamp_splitter(line));
        }
    }
}

fn timestamp_splitter<'a>(timestamp_line: &'a str) -> Vec<&'a str> {
    let timestamp_indicator = " --> ";

    timestamp_line.split(timestamp_indicator).collect()
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
    //     #[test]
    //     fn count_indexes() {
    //         let amount: u8 = 2;
    //         let example = "\
    // 1
    // 00:02:17,440 --> 00:02:20,375
    // Senator, we're making
    // our final approach into Coruscant.
    //
    // 2
    // 00:02:20,476 --> 00:02:22,501
    // Very good, Lieutenant.";
    //         assert_eq!(amount, index_counter(example));
    //     }
}
