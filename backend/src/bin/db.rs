extern crate argh;
//extern crate backend;
extern crate comfy_table;
extern crate rocket_contrib;
use rocket_contrib::databases::rusqlite;

use anyhow::{anyhow, Result};
use argh::FromArgs;
use comfy_table::Table;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use backend::database::create_tables;

#[derive(FromArgs)]
/// a tool to interact with the database
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Init(InitDb),
    Print(PrintDb),
}

/// Initialize a database
#[derive(FromArgs)]
#[argh(subcommand, name = "init")]
struct InitDb {
    #[argh(positional)]
    /// the path to the database
    path: PathBuf,
}

/// Print a table of a database
#[derive(FromArgs)]
#[argh(subcommand, name = "print")]
struct PrintDb {
    #[argh(positional)]
    /// table to print
    table: String,
    #[argh(positional)]
    /// the path to the database
    path: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();

    // This is stupid if there are more subcommands than init
    match args.subcommand {
        SubCommand::Init(v) => init_db(&v.path).unwrap(),
        SubCommand::Print(v) => print_db(&v.path, &v.table).unwrap(),
    };
}

fn init_db(path: &Path) -> Result<()> {
    let db = Connection::open(&path)?;
    create_tables(&db)?;
    println!("Created database at {:?}", path);
    Ok(())
}

fn print_db(path: &Path, table: &str) -> Result<()> {
    let conn = Connection::open(&path)?;
    let columns = match table {
        "student" => 5,
        "teacher" => 4,
        "class" => 4,
        "badge" => 5,
        _ => return Err(anyhow!("Invalid table")),
    };

    // get the stuff
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
    let rows = stmt.query_map(&[], |row| {
        let mut rowv: Vec<String> = Vec::new();
        for i in 0..columns - 1 {
            rowv.push(row.get(i))
        }
        rowv
    })?;

    // make a table
    let mut table = Table::new();
    for row in rows {
        table.add_row(row?);
    }
    println!("{}", table);
    Ok(())
}
