use rusqlite::Result;
use structopt::StructOpt;
mod cli;
mod core;
mod io;

fn main() -> Result<()> {
    let args = cli::Cli::from_args();

    let _ = handle_command(args);

    Ok(())
}

fn handle_command(cli: cli::Cli) -> Result<()> {
    match cli {
        cli::Cli::Init => {
            let _ = io::init();
            println!("Created dir");
        }
        cli::Cli::Tasks(tasks) => match tasks {
            cli::Tasks::Get(_cfg) => {
                let tasks = io::get_tasks();
                for task in &tasks.unwrap() {
                    if let Some(due) = task.due {
                        println!("[{}][{}] {}", task.id.unwrap(), due, task.name);
                    } else {
                        println!("[{}] {}", task.id.unwrap(), task.name);
                    }
                }
            }
            cli::Tasks::Add(cfg) => {
                let _ = io::add_task(cfg.message, cfg.due);
            }
            cli::Tasks::Complete(cfg) => {
                let _ = io::complete_task(cfg.id);
            }
        },
        cli::Cli::Lists(lists) => match lists {
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
