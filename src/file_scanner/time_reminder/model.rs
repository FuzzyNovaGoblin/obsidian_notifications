use chrono::{DateTime, Datelike, Local, LocalResult, TimeZone, Timelike};
use regex::{Captures, Match};
use std::fmt::Display;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct ReminderKey {
    pub file_path: String,
    pub msg: String,
    pub time_parts: DateTimeParts,
    pub completed_checked: bool,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct DateTimeParts {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,

    hour: Option<u32>,
    minute: Option<u32>,
    second: Option<u32>,
}

impl DateTimeParts {
    pub fn new(cap: &Captures<'_>) -> Result<DateTimeParts, String> {
        Ok(DateTimeParts {
            year: Self::parse_u32(cap.name("year"))?,
            month: Self::parse_u32(cap.name("month"))?,
            day: Self::parse_u32(cap.name("day"))?,
            hour: Self::parse_u32(cap.name("hour"))?,
            minute: Self::parse_u32(cap.name("minute"))?,
            second: Self::parse_u32(cap.name("second"))?,
        })
    }

    fn parse_u32(mat: Option<Match<'_>>) -> Result<Option<u32>, String> {
        match mat {
            Some(v) => match v.as_str().parse::<u32>() {
                Ok(v) => Ok(Some(v)),
                Err(e) => {
                    Err(format!(
                            "Error parsing value ({value}) as u32\nError at {file}:{line}  with message:```{msg:?}```",
                            value=v.as_str(),
                            file = file!(),
                            line = line!(),
                            msg = e
                        ))
                }
            },
            None => Ok(None),
        }
    }

    pub fn get_target_time(&self) -> LocalResult<DateTime<Local>> {
        let now = Local::now();

        let top_of_hour = now.minute() == 0 || now.minute() == 59;

        Local.with_ymd_and_hms(
            self.year.unwrap_or(now.year() as u32) as i32,
            self.month.unwrap_or(now.month()),
            self.day.unwrap_or(now.day()),
            self.hour.unwrap_or(if top_of_hour {
                now.hour()
            } else {
                now.hour() + 1
            }),
            self.minute
                .unwrap_or(if top_of_hour { now.minute() } else { 0 }),
            self.second.unwrap_or(0),
        )
    }
}

impl Display for DateTimeParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let get_str = |v: Option<u32>| match v {
            Some(v) => v.to_string(),
            None => "~".to_owned(),
        };

        write!(
            f,
            "{}-{}-{} {:0>2}:{:0>2}",
            get_str(self.year),
            get_str(self.month),
            get_str(self.day),
            get_str(self.hour),
            get_str(self.minute),
        )?;

        if let Some(s) = self.second {
            write!(f, "{:0>2}", s)?;
        }

        Ok(())
    }
}
