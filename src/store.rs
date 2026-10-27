use std::collections::HashMap;
use std::io::Error;

use crate::wal::Wal;

pub struct KvStore {
    data: HashMap<String, String>,
    wal: Wal,
}

impl KvStore {
    pub fn new() -> Result<KvStore, Error> {
        let wal = Wal::new()?;
        let mut data: HashMap<String, String> = HashMap::new();
        wal.replay(&mut data)?;

        Ok(KvStore { data, wal })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        self.wal.write_set(&key, &value)?;
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Result<Option<String>, Error> {
        self.wal.write_delete(key)?;
        Ok(self.data.remove(key))
    }

    pub fn compact(&mut self) -> Result<(), Error> {
        self.wal.compact(&self.data)
    }
}
