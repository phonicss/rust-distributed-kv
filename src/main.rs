mod store;

use store::KvStore;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


fn main() {

    

    let mut store = match KvStore::new() {
        Ok(store) => store,
        Err(error) => {
            eprintln!("Failed to create KvStore: {}", error);
            return;
        }
    };

    match store.set(
        "Name".to_string(),
        "Ivan".to_string()
    ) {
        Ok(()) => {println!("SET successful")},
        Err(error) => {
            eprintln!("SET error: {}", error);
        }
    }

    match store.get("Name") {
        Some(value) => println!("Recovered: {}", value),
        None => println!("Not found"),
    }

    match store.delete("Name") {
        Ok(Some(value)) => println!("Deleted: {}", value),
        Ok(None) => println!("Key not found"),
        Err(error) => eprintln!("DELETE error: {}", error),
    }

}
