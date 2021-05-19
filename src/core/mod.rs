use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;
use chrono::Weekday;

#[derive(Debug)]
pub struct Task {
    pub id: Option<u64>,
    pub name: String,
    pub due: Option<NaiveDate>,
    pub completed: bool,
}

#[derive(Debug)]
pub enum Due {
    Today,
    Tomorrow,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub trait NaiveDateMethods {
    fn parse_from_due(due: Due) -> Result<NaiveDate, chrono::ParseError>;
}

impl NaiveDateMethods for NaiveDate {
    fn parse_from_due(due: Due) -> Result<NaiveDate, chrono::ParseError> {
        match due {
            Due::Today => Ok(Utc::now().date().naive_utc()),
            Due::Tomorrow => Ok((Utc::now().date() + Duration::days(1)).naive_utc()),
            Due::Monday => Ok(next_weekday(Weekday::Mon)),
            Due::Tuesday => Ok(next_weekday(Weekday::Tue)),
            Due::Wednesday => Ok(next_weekday(Weekday::Wed)),
            Due::Thursday => Ok(next_weekday(Weekday::Thu)),
            Due::Friday => Ok(next_weekday(Weekday::Fri)),
            Due::Saturday => Ok(next_weekday(Weekday::Sat)),
            Due::Sunday => Ok(next_weekday(Weekday::Sun)),
        }
    }
}

//TODO optimise, fix current day and unit test
fn next_weekday(weekday: Weekday) -> NaiveDate {
    let datetime;
    let now = Utc::now().date();
    let day_of_week = now.weekday();

    if day_of_week as u32 <= weekday as u32 {
        datetime = now + Duration::days((weekday as u32 - day_of_week as u32).into());
    } else {
        datetime = now
            + Duration::weeks(1)
            + Duration::days((weekday as i32 - day_of_week as i32).into());
    }

    datetime.naive_utc()
}
