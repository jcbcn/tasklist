use rusqlite::Result;
use structopt::StructOpt;
use tasklist_server::server;

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
        }
        cli::Commands::Tasks(tasks) => match tasks {
            cli::Tasks::Get(_cfg) => {
                let tasks = db::get_tasks(_cfg.due);
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
        }
        cli::Commands::Run => {
            let _ = server::start();
        }
    }

    Ok(())
}
