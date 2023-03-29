use anyhow::{Context, Result};
use std::fs;
fn main() -> Result<()> {
    let id = read_file()?;
    println!("id : {}", id);
    Ok(())
}

#[derive(Debug)]
pub enum MyError {
    IoError(String),
    ParseError(String),
}

fn read_file() -> Result<i64> {
    let content = fs::read_to_string("number.txta").context(format!("unable to read"))?;
    let id = content.parse::<i64>()?;
    Ok(id)
}
