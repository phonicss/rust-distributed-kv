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

}
