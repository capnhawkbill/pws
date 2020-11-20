extern crate backend;
extern crate argh;
extern crate rusqlite;

use std::path::PathBuf;
use argh::FromArgs;
use rusqlite::Connection;

#[derive(FromArgs)]
/// a tool to interact with the database
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommand,
}

#[deriv(FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    /// initialize the database at specified location
    Init(PathBuf)
}

fn main() {
    let args: Args = argh::from_env();

    // This is stupid if there are more subcommands than init
    let path = match args.subcommand {
        SubCommand(path) => path,
    };

    init_db(path).unwrap();
}

fn init_db(path: PathBuf) -> Result<()> {
    let db = Connection::open(&path);
    // TODO make empty student table
    // TODO make empty teacher table
    // TODO make empty admin table
    // TODO make empty class table
    // TODO make empty badge table
}
