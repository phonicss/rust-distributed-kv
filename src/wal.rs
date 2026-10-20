use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::io::Write;


pub struct Wal {
    file: File,
}

impl Wal {
    pub fn new() -> Result<Wal, Error> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open("data.log")?;
        Ok(Wal { file })
    }

    pub fn replay(&self, data: &mut HashMap<String, String>) -> Result<(), Error> {
        let reader = BufReader::new(&self.file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
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
        Ok(())
    }

    pub fn write_set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let command = format!("SET {} {}\n", key, value);
        self.file.write_all(command.as_bytes())?;
        Ok(())
    }

    pub fn write_delete(&mut self, key: &str) -> Result<(), Error> {
        let command = format!("DELETE {}\n", key);
        self.file.write_all(command.as_bytes())?;
        Ok(())
    }

}
