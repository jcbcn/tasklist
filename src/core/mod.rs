use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::Duration;
use chrono::Weekday;
use chrono::DateTime;
use chrono::Datelike;

#[derive(Debug)]
pub struct Task {
    pub id: Option<u64>,
    pub name: String,
    pub due: Option<NaiveDateTime>,
    pub completed: bool
}

#[derive(Debug)]
pub enum Due {
    Today,
    Tomorrow,
    ThisWeek,
    NextWeek,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub trait NaiveDateTimeMethods {
    fn parse_from_due(due: Due) -> Result<NaiveDateTime, chrono::ParseError>;
}

impl NaiveDateTimeMethods for NaiveDateTime {
    fn parse_from_due(due: Due) -> Result<NaiveDateTime, chrono::ParseError> {
        match due {
            Due::Today => Ok(Utc::now().naive_utc()),
            Due::Tomorrow => Ok((Utc::now() + Duration::days(1)).naive_utc()),
            Due::Monday => Ok(nextWeekday(Weekday::Mon).naive_utc()),
            Due::Tuesday => Ok(nextWeekday(Weekday::Tue).naive_utc()),
            Due::Wednesday => Ok(nextWeekday(Weekday::Wed).naive_utc()),
            Due::Thursday => Ok(nextWeekday(Weekday::Thu).naive_utc()),
            Due::Friday => Ok(nextWeekday(Weekday::Fri).naive_utc()),
            Due::Saturday => Ok(nextWeekday(Weekday::Sat).naive_utc()),
            Due::Sunday => Ok(nextWeekday(Weekday::Sun).naive_utc()),
            Due::ThisWeek => Ok(nextWeekday(Weekday::Fri).naive_utc()),
            Due::NextWeek => Ok((nextWeekday(Weekday::Fri) + Duration::weeks(1)).naive_utc()),
            //_ => Err(())
        }
    }
}

//TODO optimise, fix current day and unit test 
fn nextWeekday(weekday: Weekday) -> DateTime<Utc> {
    let now = Utc::now();
    let dayOfWeek = now.date().weekday();

    if (dayOfWeek as u32 <= weekday as u32) { 
        // then just give me this week's instance of that day
        return now + Duration::days((weekday as u32 - dayOfWeek as u32).into());
      } else {
        // otherwise, give me *next week's* instance of that same day
        return now + Duration::weeks(1) + Duration::days((weekday as i32 - dayOfWeek as i32).into())
      }
}