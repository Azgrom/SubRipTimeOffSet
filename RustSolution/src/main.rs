use std::{env, fs, process};

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

    println!("{:?}", file.filename);
    println!("{:?}", file.contents);
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn count_indexes() {
        let amount: u8 = 2;
        let example = "\
1
00:02:17,440 --> 00:02:20,375
Senator, we're making
our final approach into Coruscant.

2
00:02:20,476 --> 00:02:22,501
Very good, Lieutenant.";
        assert_eq!(amount, index_counter(example));
    }
}
