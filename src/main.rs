use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    operation: Option<Operation>,
    object: String
}

#[derive(StructOpt)]
enum Operation {
    Get,
    Add
}

fn main() {
    let args = Cli::from_args();
    handle_subcommand(args);
}

fn handle_subcommand(cli: Cli) {
    if let Some(op) = cli.operation{
        match op {
            Operation::Get => println!("Get {}", cli.object),
            Operation::Add => println!("Add {}", cli.object)
        }
    }
}