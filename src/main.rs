use rusqlite::Result;
use structopt::StructOpt;
mod cli;
mod core;
mod db;

fn main() -> Result<()> {
    let args = cli::Commands::from_args();

    let _ = handle_command(args);

    Ok(())
}

fn handle_command(cli: cli::Commands) -> Result<()> {
    match cli {
        cli::Commands::Init => {
            let _ = db::init();
            println!("Created dir");
        }
        cli::Commands::Tasks(tasks) => match tasks {
            cli::Tasks::Get(_cfg) => {
                let tasks = db::get_tasks();
                for task in &tasks.unwrap() {
                    if let Some(due) = task.due {
                        println!("[{}][{}] {}", task.id.unwrap(), due, task.name);
                    } else {
                        println!("[{}] {}", task.id.unwrap(), task.name);
                    }
                }
            }
            cli::Tasks::Add(cfg) => {
                let _ = db::add_task(cfg.message, cfg.due);
            }
            cli::Tasks::Complete(cfg) => {
                let _ = db::complete_task(cfg.id);
            }
        },
        cli::Commands::Lists(lists) => match lists {
            cli::Lists::Get(_cfg) => {
                println!("Lists Get");
            }
            cli::Lists::Add(_cfg) => {
                println!("Lists Add");
            }
        },
    }

    Ok(())
}
