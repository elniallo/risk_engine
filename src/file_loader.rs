use std::fs::File;
use std::io::Read;
use super::account;
use serde_json;
use serde_json::error::Error;
pub fn load_dataset() -> Result<Vec<account::Account>,Box<Error>> {
    let mut file = File::open("./assets/test_data.json").unwrap();
    let val: Vec<account::Account> = serde_json::from_reader(file)?;
    Ok(val)
}