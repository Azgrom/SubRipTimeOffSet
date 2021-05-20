// pub trait Summary {
//     fn fmt(&self, sintax: T) -> String {
//         format!()
//     }
// }

pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

pub struct Dialogs {
    pub index: u32,
    pub start_time: Time,
    pub end_time: Time,
    pub dialog: String,
}

pub struct SubRipFile {
    pub subs: Vec<Dialogs>,
    pub filename: Sub
}

impl SubRip {
    pub fn new(args: &[String]) -> Result<SubRipFile, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(SupRibFile {
            subs:
        })
    }
}
