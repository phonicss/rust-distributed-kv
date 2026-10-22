mod store;
mod wal;

use store::KvStore;


fn main() {

    let mut store = match KvStore::new() {
        Ok(store) => store,
        Err(error) => {
            eprintln!("Failed to create KvStore: {}", error);
            return;
        }
    };

    match store.get("Name") {
        Some(value) => println!("Recovered after restart: {}", value),
        None => println!("Name not found"),
    }

    match store.set(("Name").to_string(), ("Ivan").to_string()) {
        Ok(()) => (),
        Err(error) => println!("Name not found"),
    }

    match store.set(("Name").to_string(), ("Alex").to_string()) {
        Ok(()) => (),
        Err(error) => println!("Name not found"),
    }

    match store.set(("Name").to_string(), ("John").to_string()) {
        Ok(()) => (),
        Err(error) => println!("Name not found"),
    }

    match store.set(("Language").to_string(), ("Rust").to_string()) {
        Ok(()) => (),
        Err(error) => println!("Name not found"),
    }

    match store.delete("Language") {
        Ok(Some(value)) => println!("Deleted: {}", value),
        Ok(None) => println!("Key not found"),
        Err(error) => eprintln!("DELETE error: {}", error),
    }

    match store.set(("Age").to_string(), ("33").to_string()) {
        Ok(()) => (),
        Err(error) => println!("Name not found"),
    }

    match store.compact() {
        Ok(()) => println!("Compaction successful"),
        Err(error) => eprintln!("Compaction error: {}", error),
    }

}
