use chrono::NaiveDate;
use structopt::StructOpt;
use tasklist_core::core::NaiveDateMethods;

#[derive(StructOpt, Debug)]
pub enum Commands {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    Run
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
pub struct GetTasksArguments {
    #[structopt(short, parse(try_from_str = parse_naive_date))]
    pub due: Option<NaiveDate>,
}

#[derive(StructOpt, Debug)]
pub struct AddTasksArguments {
    #[structopt(short)]
    pub message: String,
    #[structopt(short, parse(try_from_str = parse_naive_date))]
    pub due: Option<NaiveDate>,
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

fn parse_naive_date(src: &str) -> Result<NaiveDate, chrono::ParseError> {
    let due : Result<NaiveDate, chrono::ParseError> = match src.parse() {
        Ok(due) => NaiveDate::parse_from_due(due),
        Err(()) => NaiveDate::parse_from_str(src, "%Y-%m-%d %H:%M:%S")
    };

    due
}
