extern crate argh;
//extern crate backend;
extern crate rocket_contrib;
extern crate comfy_table;
use rocket_contrib::databases::rusqlite;

use anyhow::{anyhow, Result};
use comfy_table::Table;
use argh::FromArgs;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

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
    Print(PrintDb)
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
    /// the path to the database
    path: PathBuf,
    #[argh(positional)]
    /// table to print
    table: String,

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
    let _db = Connection::open(&path)?;
    // TODO make empty student table
    // TODO make empty teacher table
    // TODO make empty admin table
    // TODO make empty class table
    // TODO make empty badge table
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
        _ => return Err(anyhow!("Invalid table"))
    };

    // get the stuff
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
    let rows = stmt.query_map(&[], |row| {
        let mut rowv: Vec<String> = Vec::new();
        for i in 0..columns-1 {
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
