use std::collections::HashMap;
use std::f32::consts::E;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;
use std::io::{BufRead, BufReader};


pub struct KvStore {
    data: HashMap<String, String>,
    wal: File,
}

impl KvStore {

    pub fn new() -> Result<KvStore, Error> {
        let wal = match OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open("data.log") {
                Ok(file) => file,
                Err(error) => {
                    return Err(error);
            }
        };

        let mut data: HashMap<String, String> = HashMap::new();
        let reader = BufReader::new(&wal);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("{}", line);
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 3 && parts[0] == "SET" {
                        data.insert(
                            parts[1].to_string(),
                            parts[2].to_string()
                        ); 
                    } else if parts.len() == 2 && parts[0] == "DELETE" {
                        data.remove(parts[1]);
                    } else {
                        println!("Invalid WAL entry");
                    }
                
                }
                Err(error) => {
                    eprintln!("Error reading line: {}", error);
                    return Err(error);
                }
            }
        }

        Ok(KvStore {
            data: data,
            wal: wal,
        })

        
    }
    
    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        let command = format!("SET {} {}\n", key, value);
        match self.wal.write_all(command.as_bytes()) {
            Ok(()) => {},
            Err(error) => {
                return Err(error);
            }   
        };
        self.data.insert(key,value);
        Ok(())
    }


    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Result<Option<String>, Error> {
        let command = format!("DELETE {}\n", key);
        match self.wal.write_all(command.as_bytes()) {
            Ok(()) => {},
            Err(error) => {
                return Err(error);
            }
        };
        Ok(self.data.remove(key))
        

        
        
    }
}
