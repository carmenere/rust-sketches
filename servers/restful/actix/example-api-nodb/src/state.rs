use std::sync::Mutex;
use std::collections::HashMap;

pub struct PersonsTable {
    pub id: Mutex<u64>,
    pub table: Mutex<HashMap<u64, crate::models::Person>>
}

pub struct Db {
    pub persons: PersonsTable
}

pub fn init() -> Db {
    Db {
        persons: PersonsTable { 
            id: Mutex::new(0),
            table: Mutex::new(HashMap::with_capacity(16)) 
        },
    }
}