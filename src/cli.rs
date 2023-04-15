use clap::{ AppSettings, Clap };

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "status")] Status(Status),
    #[clap(name = "list")] List(List),
    #[clap(name = "add")] Add(Add),
    #[clap(name = "remove")] Remove(Remove),
    #[clap(name = "refresh")] Refresh(Refresh),
}

#[derive(Clap)]
struct Status {
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Clap)]
struct List {}

#[derive(Clap)]
struct Add {
    #[clap(short, long)]
    name: String,
}

#[derive(Clap)]
struct Remove {
    #[clap(short, long)]
    name: String,
}

#[derive(Clap)]
struct Refresh {}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Status(status) => {
            println!("Running status command");
            if status.verbose {
                println!("Verbose mode is on");
            }
        }
        SubCommand::List(list) => {
            println!("Running list command");
        }
        SubCommand::Add(add) => {
            println!("Running add command with name: {}", add.name);
        }
        SubCommand::Remove(remove) => {
            println!("Running remove command with name: {}", remove.name);
        }
        SubCommand::Refresh(refresh) => {
            println!("Running refresh command");
        }
    }
}