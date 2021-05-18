use chrono::NaiveDateTime;
use structopt::StructOpt;
use std::str::FromStr;
use crate::core::Due;
use crate::core::NaiveDateTimeMethods;

#[derive(StructOpt, Debug)]
pub enum Commands {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    #[structopt(alias = "l")]
    Lists(Lists),
}

#[derive(StructOpt, Debug)]
pub enum Tasks {
    #[structopt(name = "get", alias = "g")]
    Get(GetTasksArguments),
    #[structopt(name = "add", alias = "a")]
    Add(AddTasksArguments),
    #[structopt(name = "complete", alias = "c")]
    Complete(CompleteTasksArguments),
}

#[derive(StructOpt, Debug)]
pub enum Lists {
    #[structopt(name = "get", alias = "g")]
    Get(GetListsArguments),
    #[structopt(name = "add", alias = "a")]
    Add(AddListsArguments),
}

#[derive(StructOpt, Debug)]
pub struct GetTasksArguments {
    #[structopt(short, parse(try_from_str = parse_naivedatetime))]
    pub due: Option<NaiveDateTime>,
}

#[derive(StructOpt, Debug)]
pub struct AddTasksArguments {
    #[structopt(short)]
    pub message: String,
    #[structopt(short, parse(try_from_str = parse_naivedatetime))]
    pub due: Option<NaiveDateTime>,
    #[structopt(short = "rd")]
    pub recurs_daily: Option<bool>,
}

#[derive(StructOpt, Debug)]
pub struct CompleteTasksArguments {
    #[structopt(short)]
    pub id: u64,
}

#[derive(StructOpt, Debug)]
pub struct GetListsArguments {}

#[derive(StructOpt, Debug)]
pub struct AddListsArguments {}

fn parse_naivedatetime(src: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    let due : Result<NaiveDateTime, chrono::ParseError> = match src.parse() {
        Ok(due) => NaiveDateTime::parse_from_due(due),
        Err(()) => NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")
    };

    due
}

impl FromStr for Due {
    type Err = ();

    fn from_str(input: &str) -> Result<Due, Self::Err> {
        match input.to_lowercase().as_str() { //TODO performance issue?
            "today"  => Ok(Due::Today),
            "td" => Ok(Due::Today),
            "tomorrow"  => Ok(Due::Tomorrow),
            "tm"  => Ok(Due::Tomorrow),
            "thisWeek"  => Ok(Due::ThisWeek),
            "tw"  => Ok(Due::ThisWeek),
            "nextWeek"  => Ok(Due::NextWeek),
            "nw"  => Ok(Due::NextWeek),
            "monday"  => Ok(Due::Monday),
            "mon"  => Ok(Due::Monday),
            "tuesday"  => Ok(Due::Tuesday),
            "tue"  => Ok(Due::Tuesday),
            "wednesday"  => Ok(Due::Wednesday),
            "wed"  => Ok(Due::Wednesday),
            "thursday"  => Ok(Due::Thursday),
            "thu"  => Ok(Due::Thursday),
            "friday"  => Ok(Due::Friday),
            "fri"  => Ok(Due::Friday),
            "saturday"  => Ok(Due::Saturday),
            "sat"  => Ok(Due::Saturday),
            "sunday"  => Ok(Due::Sunday),
            "sun"  => Ok(Due::Sunday),
            _      => Err(()),
        }
    }
}