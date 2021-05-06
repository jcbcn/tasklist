use chrono::NaiveDateTime;
use structopt::StructOpt;

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
    #[structopt(short)]
    due: Option<String>,
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
    //check if Due enum
    NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")
}
