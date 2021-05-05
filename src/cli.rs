use chrono::NaiveDateTime;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    #[structopt(alias = "l")]
    Lists(Lists),
}

#[derive(StructOpt, Debug)]
pub enum Tasks {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
    #[structopt(name = "complete", alias = "c")]
    Complete(CompleteOperation),
}

#[derive(StructOpt, Debug)]
pub enum Lists {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
}

#[derive(StructOpt, Debug)]
pub struct GetOperation {
    #[structopt(short)]
    due: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct AddOperation {
    #[structopt(short)]
    pub message: String,
    #[structopt(short, parse(try_from_str = parse_naivedatetime))]
    pub due: Option<NaiveDateTime>,
    #[structopt(short = "rd")]
    pub recurs_daily: Option<bool>,
}

#[derive(StructOpt, Debug)]
pub struct CompleteOperation {
    #[structopt(short)]
    pub id: u64,
}

fn parse_naivedatetime(src: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    //check if Due enum
    NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")
}
