mod command;
mod store;
mod wal;

use command::Command;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use store::KvStore;

fn handle_client(stream: TcpStream, store: Arc<RwLock<KvStore>>) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(stream);
    let mut input = String::new();
    loop {
        input.clear();
        let bytes_read = reader.read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }
        println!("Received: {}", input.trim());
        match Command::parse(input.trim()) {
            Ok(command) => match command {
                Command::Get(key) => {
                    let value = {
                        let guard = store.read().unwrap();
                        match guard.get(&key) {
                            Some(value) => Some(value.clone()),
                            None => None,
                        }
                    };

                    match value {
                        Some(value) => {
                            let response = format!("VALUE {}\n", value);
                            reader.get_mut().write_all(response.as_bytes())?;
                        }
                        None => {
                            println!("NOT_FOUND");
                            reader.get_mut().write_all(b"NOT_FOUND\n")?;
                        }
                    }
                }
                Command::Set { key, value } => {
                    let result = {
                        let mut guard = store.write().unwrap();
                        guard.set(key, value)
                    };
                    match result {
                        Ok(()) => {
                            println!("Ok");
                            reader.get_mut().write_all(b"OK\n")?;
                        }
                        Err(error) => {
                            println!("SET error: {}", error);
                            let response = format!("ERROR: {}\n", error);
                            reader.get_mut().write_all(response.as_bytes())?;
                        }
                    }
                }
                Command::Delete(key) => {
                    let result = {
                        let mut guard = store.write().unwrap();
                        guard.delete(&key)
                    };

                    match result {
                        Ok(Some(value)) => {
                            println!("Deleted: {}", value);
                            let response = format!("DELETED {}\n", value);
                            reader.get_mut().write_all(response.as_bytes())?;
                        }
                        Ok(None) => {
                            println!("NOT_FOUND");
                            reader.get_mut().write_all(b"NOT_FOUND\n")?;
                        }
                        Err(error) => {
                            println!("Error {}", error);
                            let response = format!("ERROR: {}\n", error);
                            reader.get_mut().write_all(response.as_bytes())?;
                        }
                    }
                }
            },
            Err(error) => {
                eprintln!("Parse error: {}", error);
                let response = format!("ERROR: {}\n", error);
                reader.get_mut().write_all(response.as_bytes())?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("KV server listening on 127.0.0.1:4000");
    let store = Arc::new(RwLock::new(KvStore::new()?));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || match handle_client(stream, store) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Client error: {}", error);
                    }
                });
            }
            Err(error) => {
                eprintln!("Connection error: {}", error);
            }
        }
    }
    Ok(())
}
