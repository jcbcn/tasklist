use chrono::NaiveDateTime;
use rusqlite::Result;
use structopt::StructOpt;
mod core;
mod io;

#[derive(StructOpt, Debug)]
enum Cli {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    #[structopt(alias = "l")]
    Lists(Lists),
}

#[derive(StructOpt, Debug)]
enum Tasks {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
    #[structopt(name = "complete", alias = "c")]
    Complete(CompleteOperation),
}

#[derive(StructOpt, Debug)]
enum Lists {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
}

#[derive(StructOpt, Debug)]
struct GetOperation {
    #[structopt(short)]
    due: Option<String>,
}

#[derive(StructOpt, Debug)]
struct AddOperation {
    #[structopt(short)]
    message: String,
    #[structopt(short, parse(try_from_str = parse_naivedatetime))]
    due: Option<NaiveDateTime>,
    #[structopt(short = "rd")]
    recurs_daily: Option<bool>,
}

#[derive(StructOpt, Debug)]
struct CompleteOperation {
    #[structopt(short)]
    id: u64,
}


fn parse_naivedatetime(src: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    //check if Due enum
    NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")
}


fn main() -> Result<()> {
    let args = Cli::from_args();

    let _ = handle_subcommand(args);

    Ok(())
}

fn handle_subcommand(cli: Cli) -> Result<()> {
    match cli {
        Cli::Init => {
            let _ = io::init();
            println!("Created dir");
        }
        Cli::Tasks(tasks) => match tasks {
            Tasks::Get(_cfg) => {
                let tasks = io::get_tasks();
                for task in &tasks.unwrap() {
                    if let Some(due) = task.due {
                        println!("[{}][{}] {}", task.id.unwrap(), due, task.name);
                    } else {
                        println!("[{}] {}", task.id.unwrap(), task.name);
                    }
                }
            }
            Tasks::Add(cfg) => {
                let _ = io::add_task(cfg.message, cfg.due);
            },
            Tasks::Complete(cfg) => {
                let _ = io::complete_task(cfg.id);
            }
        },
        Cli::Lists(lists) => match lists {
            Lists::Get(_cfg) => {
                println!("Lists Get");
            }
            Lists::Add(_cfg) => {
                println!("Lists Add");
            }
        },
    }

    Ok(())
}
