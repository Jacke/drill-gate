#![allow(unused)]
use clap::{ Parser, Subcommand };
use human_panic::*;
use anyhow::{ Context, Result };
use uuid::Uuid;
use env_logger;
use log::{ debug, error };
mod db;
use db::get_connection;
mod persistence;

#[derive(Parser)] // requires `derive` feature
#[command(name = "drills-gate")]
#[command(bin_name = "drills-gate")]
enum CargoCli {
    ExampleDerive(ExampleDeriveArgs),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add {
        title: Option<String>,
    },
    Remove {
        id: Option<String>,
    },
    Update {
        id: Option<String>,
    },
    Status,
    List,
    ReinitDB,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct ExampleDeriveArgs {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

fn main() {
    static DB_NAME: &str = "entries.db";
    setup_panic!();
    // let CargoCli::ExampleDerive(args) = CargoCli::parse();
    // println!("{:?}", args.manifest_path);
    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();
    let conn = get_connection(DB_NAME).unwrap();
    persistence::init_table(&conn);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { title } => {
            log::debug!("Engine temperature is 200 degrees");
            persistence::add_entry(&conn, title.to_owned());
            println!("'myapp add' was used, name is: {title:?}")
        }
        Commands::Remove { id } => {
            let uuid = Uuid::parse_str(id.to_owned().unwrap_or_default().as_str()).unwrap();
            persistence::remove_entry(&conn, uuid);
        }
        Commands::Update { id } => {
            let uuid = Uuid::parse_str(id.to_owned().unwrap_or_default().as_str()).unwrap();
            persistence::update_entry(&conn, uuid, serde_json::Value::Object(serde_json::Map::new()));
        }
        Commands::Status => { println!("Commands::Status") }
        Commands::List => {
            use dialoguer::{console::Term, theme::ColorfulTheme, Select};
            let items = vec!["Item 1", "item 2"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr()).unwrap();

            println!("Commands::List, {:?}", selection);
            let entries = persistence::list_entries(&conn).unwrap();
            let json = serde_json::to_string_pretty(&entries).unwrap();
            println!("entries: {}",json);
        }
        Commands::ReinitDB => {
            persistence::drop_table(&conn);
            persistence::init_table(&conn);
            println!("DB {} has been reinitialized", DB_NAME);
        }
    }
}