use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;


#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn main() -> Result<()> {
let path = "/test.db";
let db = Connection::open(&path)?;
println!(" {} ", db.is_autocommit());
     Ok(())
}

